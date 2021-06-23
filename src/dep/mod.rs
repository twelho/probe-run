//! Dependency path parsing

use std::{
    ffi::OsStr,
    path::{Component, Path as StdPath},
};

mod cratesio;
mod rust_repo;
mod rust_std;
mod rustc;

#[derive(Debug, PartialEq)]
pub(crate) enum Path<'p> {
    Cratesio(cratesio::Path<'p>),
    /// Path into `rust-std` component
    RustStd(rust_std::Path<'p>),
    /// "Remapped" rust-lang/rust path AKA `/rustc` path
    Rustc(rustc::Path<'p>),
    Verbatim(&'p StdPath),
}

impl<'p> Path<'p> {
    pub(crate) fn from_std_path<T>(path: &'p T) -> Self
    where
        T: AsRef<StdPath>,
    {
        if let Some(rust_std) = rust_std::Path::from_std_path(path.as_ref()) {
            Self::RustStd(rust_std)
        } else if let Some(rustc) = rustc::Path::from_std_path(path.as_ref()) {
            Self::Rustc(rustc)
        } else if let Some(cratesio) = cratesio::Path::from_std_path(path.as_ref()) {
            Self::Cratesio(cratesio)
        } else {
            Self::Verbatim(path.as_ref())
        }
    }

    pub(crate) fn format_short(&self) -> String {
        match self {
            Path::Cratesio(cratesio) => cratesio.format_short(),
            Path::RustStd(rust_std) => rust_std.format_short(),
            Path::Rustc(rustc) => rustc.format_short(),
            Path::Verbatim(path) => path.display().to_string(),
        }
    }

    pub(crate) fn format_highlight(&self) -> String {
        match self {
            Path::Cratesio(cratesio) => cratesio.format_highlight(),
            Path::RustStd(rust_std) => rust_std.format_highlight(),
            Path::Rustc(rustc) => rustc.format_highlight(),
            Path::Verbatim(path) => path.display().to_string(),
        }
    }
}

fn get_component_normal(component: Component) -> Option<&OsStr> {
    if let Component::Normal(string) = component {
        Some(string)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    #[test]
    fn from_std_path_returns_correct_variant() {
        let cratesio = &format!(
            "{s}home{s}user{s}.cargo{s}registry{s}src{s}github.com-1ecc6299db9ec823{s}cortex-m-rt-0.6.13{s}src{s}lib.rs",
            s=path::MAIN_SEPARATOR,
        );
        let rustc = &format!(
            "{s}rustc{s}9bc8c42bb2f19e745a63f3445f1ac248fb015e53{s}library{s}core{s}src{s}panicking.rs", 
            s=path::MAIN_SEPARATOR
        );
        let rust_std = &format!(
            "{s}home{s}user{s}.rustup{s}toolchains{s}stable-x86_64-unknown-linux-gnu{s}lib{s}rustlib{s}src{s}rust{s}library{s}core{s}src{s}sync{s}atomic.rs",
            s=path::MAIN_SEPARATOR,
        );
        let local = &format!("src{s}lib.rs", s = path::MAIN_SEPARATOR);

        assert!(matches!(Path::from_std_path(cratesio), Path::Cratesio(_)));
        assert!(matches!(Path::from_std_path(rustc), Path::Rustc(_)));
        assert!(matches!(Path::from_std_path(rust_std), Path::RustStd(_)));
        assert!(matches!(Path::from_std_path(local), Path::Verbatim(_)));
    }
}
