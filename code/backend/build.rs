fn main() {
    tonic_build::configure()
        .out_dir("src/features/login/proto")
        .compile(&["proto/auth.proto"], &["proto"])
        .unwrap();
}
