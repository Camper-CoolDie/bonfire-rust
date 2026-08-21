use std::error::Error;
use std::path::PathBuf;

use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    compile_protos()?;
    Ok(())
}

fn compile_protos() -> Result<(), Box<dyn Error>> {
    let source_dir = PathBuf::from("src/proto");
    println!("cargo::rerun-if-changed={}", source_dir.display());

    let paths = WalkDir::new(&source_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_owned())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "proto")
        })
        .collect::<Vec<_>>();

    prost_build::compile_protos(&paths, &[source_dir])
        .map_err(|error| format!("failed to compile Protobuf files: {error:?}"))?;

    Ok(())
}
