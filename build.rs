fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the .proto file and generate Rust bindings for tonic/prost.
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile(&["proto/inference.proto"], &["proto"])?;

    // Re-run this build script if the proto definition changes.
    println!("cargo:rerun-if-changed=proto/inference.proto");

    Ok(())
}
