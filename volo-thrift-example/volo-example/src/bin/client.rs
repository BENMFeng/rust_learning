use lazy_static::lazy_static;
use std::net::SocketAddr;

use volo_example::LogLayer;

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .layer_inner(LogLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    let req = volo_gen::volo::example::GetItemRequest { id: 1024 };
    let resp = CLIENT.clone().get_item(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
}