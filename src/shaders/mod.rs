macro_rules! include_shader {
    ($filename:literal) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $filename, ".spv"))
    }
}