fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .out_dir("gen")
        .compile(&["proto/customer.proto"], &["proto"])
        .unwrap();

    Ok(())
}
