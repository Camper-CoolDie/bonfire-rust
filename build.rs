use std::path::PathBuf;
use std::{env, fs};

use graphql_minify::minify;

const GRAPHQL_DIR: &str = "src/queries/graphql";

fn main() {
    println!("cargo::rerun-if-changed={GRAPHQL_DIR}");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for entry in fs::read_dir(GRAPHQL_DIR).unwrap() {
        let dir = entry.unwrap().path();
        let relative = dir.strip_prefix("src").unwrap();
        fs::create_dir_all(out_dir.join(relative)).unwrap();

        for entry in fs::read_dir(dir).unwrap() {
            let src = entry.unwrap().path();
            let relative = src.strip_prefix("src").unwrap();
            let out = out_dir.join(relative);

            let content = fs::read_to_string(src).unwrap();
            let minified = minify(content).unwrap();
            fs::write(out, minified).unwrap();
        }
    }
}
