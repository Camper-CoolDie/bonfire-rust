use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

use graphql_minify::minify;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    minify_graphql()?;
    #[cfg(feature = "fcm")]
    compile_protos()?;

    Ok(())
}

fn minify_graphql() -> Result<(), Box<dyn Error>> {
    let source_dir = PathBuf::from("src/graphql");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?).join("graphql");
    println!("cargo::rerun-if-changed={}", source_dir.display());

    for entry in WalkDir::new(&source_dir)
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "graphql")
        })
    {
        let source = entry.path();
        let out = out_dir.join(source.strip_prefix(&source_dir)?);

        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create parent directories for {:?}: {}",
                    out, error
                )
            })?;
        }

        let query = fs::read_to_string(source)
            .map_err(|error| format!("failed to read GraphQL file {:?}: {}", source, error))?;
        let minified = minify(query)
            .map_err(|error| format!("failed to minify GraphQL file {:?}: {:?}", source, error))?;
        fs::write(&out, minified)
            .map_err(|error| format!("failed to write minified GraphQL to {:?}: {}", out, error))?;
    }

    Ok(())
}

#[cfg(feature = "fcm")]
fn compile_protos() -> Result<(), Box<dyn Error>> {
    let source_dir = PathBuf::from("src/proto");
    println!("cargo::rerun-if-changed={}", source_dir.display());

    let paths = WalkDir::new(&source_dir)
        .into_iter()
        .filter_map(|result| result.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_owned())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "proto")
        })
        .collect::<Vec<_>>();

    prost_build::compile_protos(&paths, &[source_dir])
        .map_err(|error| format!("failed to compile Protobuf files: {:?}", error))?;

    Ok(())
}
