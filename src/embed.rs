//! Optionally embed files into the binry at compile time.

use std::{borrow::Cow, path::Path};

/// Either the path to a real fileor the bytes of an embedded one
#[derive(Clone, Copy)]
pub enum MaybeFile {
    File(&'static str),
    Embedded(&'static [u8], &'static str),
}

impl MaybeFile {
    /// Read the contents, possibly a noop
    pub fn read(&self) -> std::io::Result<Cow<'static, [u8]>> {
        match self {
            MaybeFile::File(path) => std::fs::read(path).map(Cow::Owned),
            MaybeFile::Embedded(embedded, _) => Ok(Cow::Borrowed(embedded)),
        }
    }
    /// The path of the file
    pub fn path(&self) -> &'static Path {
        match *self {
            MaybeFile::File(p) => p.as_ref(),
            MaybeFile::Embedded(_, p) => p.as_ref(),
        }
    }
    /// The path of the file as a string
    pub fn as_str(&self) -> &'static str {
        match *self {
            MaybeFile::File(p) => p,
            MaybeFile::Embedded(_, p) => p,
        }
    }
    /// Is this file embedded in the binary?
    pub fn is_embedded(&self) -> bool {
        match self {
            MaybeFile::File(_) => false,
            MaybeFile::Embedded(_, _) => true,
        }
    }
}

/// Either creates a runtime [`MaybeFile::File`] or embeds it directly into the binary depending on whether the `embed` feature is enabled
macro_rules! maybe_embed_file {
    ($path:literal) => {
        maybe_embed_file!(@internal concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
    };
    (@internal $path:expr) => {{
        #[cfg(feature = "embed")]
        let a = MaybeFile::Embedded(include_bytes!($path), $path);
        #[cfg(not(feature = "embed"))]
        let a = MaybeFile::File($path);
        a
    }};
}
pub(crate) use maybe_embed_file;
