fn main() {
    tonic_build::configure()
        .build_client(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &["../proto/message.proto", "../proto/user.proto"],
            // this is where our proto files are
            &["../proto"],
        )
        .unwrap_or_else(|e| panic!("unable to compile proto's due to {e:?}"))
}
