use std::io::Result;
fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();
    prost_build
        // Derive serde Serialize/Deserialize so we can convert to JSON. However, there are some types that
        // cannot be converted to JSON (https://github.com/tokio-rs/prost/issues/75). Official JSON support
        // is expected in v1.0 (https://github.com/tokio-rs/prost/issues/624).
        // .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        // .compile_well_known_types()
        .compile_protos(
            &[
                "libraries/googleapis/protos/google/type/date.proto",
                "libraries/googleapis/protos/google/type/decimal.proto",
                "libraries/googleapis/protos/google/type/money.proto",
            ],
            &["../../"],
        )?;
    Ok(())
}
