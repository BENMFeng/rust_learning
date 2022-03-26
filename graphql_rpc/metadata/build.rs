fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix("capnp")
        .file("capnp/starwars.capnp")
        .run().expect("failed to compile schema");
}
