fn main() {
    // Generate protobuf code
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["protobuf/base_names.proto"],
            &["protobuf"],
        )
        .unwrap();
} 