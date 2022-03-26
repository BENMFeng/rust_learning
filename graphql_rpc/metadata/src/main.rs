mod rpc;
pub mod starwars_capnp {
    include!(concat!(env!("OUT_DIR"), "/starwars_capnp.rs"));
}

fn main() {
    rpc::server::run().expect("cannot run server")
}
