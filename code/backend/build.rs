fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/core/generated")
        .compile_protos(&["src/core/proto/auth.proto"], &["src/core/proto"])
        .unwrap();
}
