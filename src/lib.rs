#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::open_with_params;

#[cfg(target_os = "linux")]
mod gtk3;
#[cfg(target_os = "linux")]
pub use gtk3::open_with_params;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::open_with_params;

pub fn open() -> Option<std::path::PathBuf> {
    open_with_params(DialogParams::new())
}

/// Paramaters to pass to the file dialog.
#[derive(Default)]
pub struct DialogParams<'a> {
    pub filters: &'a [(&'a str, &'a str)],
}

impl<'a> DialogParams<'a> {
    /// Creates a new `DialogParams` with nothing configured.
    pub fn new() -> Self {
        Self { filters: &[] }
    }

    /// Sets the filters of this `DialogParams`.
    pub fn set_filters(mut self, filters: &'a [(&'a str, &'a str)]) -> Self {
        self.filters = filters;
        self
    }
}
