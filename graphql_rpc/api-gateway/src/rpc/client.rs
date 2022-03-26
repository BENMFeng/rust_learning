use crate::graphql::schema::{Episode, Human, NewHuman};
use crate::starwars_capnp;
use capnp_rpc::RpcSystem;
use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::rpc_twoparty_capnp::Side;
use futures::{AsyncReadExt, FutureExt};
use futures::task::{LocalSpawnExt};
use juniper::FieldResult;

pub fn show_human(id: String) -> FieldResult<Human> {
    let mut exec = futures::executor::LocalPool::new();
    let spawner = exec.spawner();

    exec.run_until(async move {
        let mut rpc_system: RpcSystem<Side> = get_rpc_system().await?;
        let starwars_client: starwars_capnp::star_wars::Client = rpc_system.bootstrap(Side::Server);

        spawner.spawn_local(Box::pin(rpc_system.map(|_|())))?;

        // Create get_result request object
        let mut request = starwars_client.show_human_request();

        // Set Human ID
        request.get().set_id(&id);

        // Send request, and await response
        let response = request.send().promise.await?;

        Ok(Human {
            id,
            name: response.get().unwrap().get_name().unwrap().to_string(),
            appears_in: appears_in_from_capnp(response.get().unwrap().get_appears_in().unwrap()),
            home_planet: response.get().unwrap().get_home_planet().unwrap().to_string(),
        })
    })
}

pub fn create_human(new_human: NewHuman) -> FieldResult<Human> {
    let mut exec = futures::executor::LocalPool::new();
    let spawner = exec.spawner();

    exec.run_until(async move {
        let mut rpc_system: RpcSystem<Side> = get_rpc_system().await?;
        let starwars_client: starwars_capnp::star_wars::Client = rpc_system.bootstrap(Side::Server);

        spawner.spawn_local(Box::pin(rpc_system.map(|_|())))?;

        // Create get_result request object
        let mut request = starwars_client.create_human_request();

        // Set Human fields
        request.get().set_name(&new_human.name);
        request.get().set_home_planet(&new_human.home_planet);
        request.get().set_appears_in(appears_in_to_capnp(new_human.appears_in));

        // Send request, and await response
        let response = request.send().promise.await?;

        Ok(Human {
            id: response.get().unwrap().get_id().unwrap().to_string(),
            name: response.get().unwrap().get_name().unwrap().to_string(),
            appears_in: appears_in_from_capnp(response.get().unwrap().get_appears_in().unwrap()),
            home_planet: response.get().unwrap().get_home_planet().unwrap().to_string(),
        })
    })
}

fn appears_in_from_capnp(appears_in: starwars_capnp::star_wars::human::AppearsIn) -> Episode {
    match appears_in {
        starwars_capnp::star_wars::human::AppearsIn::NewHope => Episode::NewHope,
        starwars_capnp::star_wars::human::AppearsIn::Empire => Episode::Empire,
        starwars_capnp::star_wars::human::AppearsIn::Jedi => Episode::Jedi,
    }
}

fn appears_in_to_capnp(appears_in: Episode) -> starwars_capnp::star_wars::human::AppearsIn {
    match appears_in {
        Episode::NewHope => starwars_capnp::star_wars::human::AppearsIn::NewHope,
        Episode::Empire => starwars_capnp::star_wars::human::AppearsIn::Empire,
        Episode::Jedi => starwars_capnp::star_wars::human::AppearsIn::Jedi,
    }
}

async fn get_rpc_system() -> Result<RpcSystem<Side>, Box<dyn std::error::Error>> {
    let stream = async_std::net::TcpStream::connect("127.0.0.1:8001").await?;

    stream.set_nodelay(true)?;

    let (reader, writer) = stream.split();
    let network = Box::new(
        VatNetwork::new(reader, writer, Side::Client, Default::default())
    );

    Ok(RpcSystem::new(network, None))
}
