use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    gossipsub::{ Gossipsub, GossipsubEvent, GossipsubConfig, MessageAuthenticity, IdentTopic },
    identity,
    mdns::{ Mdns, MdnsEvent },
    noise,
    swarm::{ NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent },
    tcp::TokioTcpConfig,
    yamux, NetworkBehaviour, PeerId, Swarm, Transport,
};
use std::borrow::Cow;
use tokio::io::{ stdin, AsyncBufReadExt, BufReader };

/// 处理 p2p 网络的 behaviour 数据结构
/// 里面的每个域都需要实现 NetworkBehaviour, 或者使用 #[behaviour(ignored)]
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct ChatBehavior {
    gossipsub: Gossipsub,
    /// 本地节点发现机制
    mdns: Mdns,
    // 在 behavior 结构中，你也可以放其他数据，但需要 ignore
    // #[behaviour(ignore)]
    // _useless: String
}

impl ChatBehavior {
    /// 创建一个新的 ChatBehavior
    pub async fn new(privacy: MessageAuthenticity, config: GossipsubConfig) -> Result<Self> {
        Ok(Self {
            gossipsub: Gossipsub::new(privacy, config).unwrap(),
            mdns: Mdns::new(Default::default()).await?,
        })
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for ChatBehavior {
    // 处理 gossipsub 产生的消息
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message{propagation_source, message_id:_, message} = event {
            let text = String::from_utf8_lossy(&message.data);
            println!("{:?}: {:?}", propagation_source, text);
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for ChatBehavior {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                // 把 mdns 发现的新的 peer 加入到 gossipsub 的 view 中
                for (id, addr) in list {
                    println!("Got peer: {} with addr {}", &id, &addr);
                    self.gossipsub.add_explicit_peer(&id);
                }
            }
            MdnsEvent::Expired(list) => {
                // 把 mdns 发现的离开的 peer 加入到 gossipsub 的 view 中
                for(id, addr) in list {
                    println!("Remove peer: {} with addr {}", &id, &addr);
                    self.gossipsub.remove_explicit_peer(&id);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 如果带参数，会当成一个 topic
    let name = match std::env::args().nth(1) {
        Some(arg) => Cow::Owned(arg),
        None => Cow::Borrowed("lobby"),
    };

    // 创建一个 gossipsub topic
    let topic = IdentTopic::new(name);

    // 创建 swarm
    let mut swarm = create_swarm(topic.clone()).await?;

    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

    // 获取 stdin 的每一行
    let mut stdin = BufReader::new(stdin()).lines();

    // main loop
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                swarm.behaviour_mut().gossipsub.publish(topic.clone(), line.as_bytes()).unwrap();
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address,.. } = event {
                    println!("Listening on {:?}", address);
                }
            }
        }
    }
}

async fn create_swarm(topic: IdentTopic) -> Result<Swarm<ChatBehavior>> {
    // 创建 identity(密钥对)
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // 使用 noise protocol 来处理加密和认证
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new().into_authentic(&id_keys)?;

    // 创建传输层
    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let msg_auth = MessageAuthenticity::Signed(id_keys);
    let gos_cfg = GossipsubConfig::default();
    // 创建 chat behavior
    let mut behavior = ChatBehavior::new(msg_auth, gos_cfg).await?;

    // 订阅某个主题
    behavior.gossipsub.subscribe(&topic).unwrap();

    // 创建 swarm
    let swarm = SwarmBuilder::new(transport, behavior, peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        })).build();

    Ok(swarm)
}