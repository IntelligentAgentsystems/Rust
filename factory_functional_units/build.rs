fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/server")
        .compile(&["proto/functional_units.proto"], &["proto"])?;
    Ok(())
}
