use glsl::parser::Parse;

fn build() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/shaders");

    let out_dir = std::env::var("OUT_DIR")?;

    std::fs::create_dir_all(&out_dir)?;

    for entry in std::fs::read_dir("src/shaders")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            let shader_type = in_path.extension().and_then(|ext| {
                match ext.to_string_lossy().as_ref() {
                    "vert" => Some(glsl::transpiler::spirv::ShaderKind::Vertex),
                    "frag" => Some(glsl::transpiler::spirv::ShaderKind::Fragment),
                    _ => None,
                }
            });

            if let Some(shader_type) = shader_type {
                let source = std::fs::read_to_string(&in_path)?;

                let out_path = format!(
                    "{}/{}.spv",
                    &out_dir,
                    in_path.file_name().unwrap().to_string_lossy()
                );
                println!("{:?} -> {}", in_path.file_name().unwrap(), out_path);

                let mut out = std::fs::File::create(out_path).unwrap();

                let ast = glsl::syntax::ShaderStage::parse(source)?;
                glsl::transpiler::spirv::transpile_translation_unit_to_binary(&mut out, &ast, shader_type)?;
            }
        }
    }

    Ok(())
}

fn main() {
    match build() {
        Ok(()) => {}
        Err(e) => { println!("{}", e) }
    }
}