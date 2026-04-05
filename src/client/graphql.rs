use include_dir::{Dir, include_dir};

static DIR: Dir = include_dir!("$OUT_DIR/graphql");

pub(super) fn contents(path: &'static str) -> &'static str {
    // These errors can be caught during testing
    DIR.get_file(path)
        .expect("no valid GraphQL file found")
        .contents_utf8()
        .expect("failed to retrieve GraphQL contents")
}
