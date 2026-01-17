mod melior;
mod root;

pub(super) use melior::MeliorService;
use once_cell::sync::Lazy;
pub(super) use root::RootService;

static USER_AGENT: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/{} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        os_info::get()
    )
});
