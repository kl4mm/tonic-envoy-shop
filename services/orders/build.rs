fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/orders.proto")?;
    tonic_build::compile_protos("proto/users.proto")?;
    Ok(())
}
