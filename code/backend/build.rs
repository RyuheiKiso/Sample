fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/generated")
        .compile(&["src/proto/auth.proto"], &["src/proto"])
        .unwrap();
}
