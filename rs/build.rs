fn main() {
    tonic_build::configure()
        .build_client(true)
        .compile(&[
            "../proto/msg.proto", 
            "../proto/user.proto"
        ], 
        // this is where our proto files are
&["../proto"])
        .unwrap_or_else(|e| panic!("unable to compile proto's due to {e:?}"))
}
