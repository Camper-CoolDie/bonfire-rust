mod melior;
mod root;

use std::sync::LazyLock;

pub(super) use melior::Service as MeliorService;
pub(super) use root::Service as RootService;

static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}/{} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        os_info::get()
    )
});
