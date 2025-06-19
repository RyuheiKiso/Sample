fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/core/generated/generated")
        .compile(&["src/core/proto/proto/auth.proto"], &["src/core/proto/proto"])
        .unwrap();
}
