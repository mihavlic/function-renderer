use std::{borrow::Cow, path::Path};

#[derive(Clone, Copy)]
pub enum MaybeFile {
    File(&'static str),
    Embedded(&'static [u8], &'static str),
}

impl MaybeFile {
    pub fn read(&self) -> std::io::Result<Cow<'static, [u8]>> {
        match self {
            MaybeFile::File(path) => std::fs::read(path).map(Cow::Owned),
            MaybeFile::Embedded(embedded, _) => Ok(Cow::Borrowed(embedded)),
        }
    }
    pub fn path(&self) -> &'static Path {
        match *self {
            MaybeFile::File(p) => p.as_ref(),
            MaybeFile::Embedded(_, p) => p.as_ref(),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match *self {
            MaybeFile::File(p) => p,
            MaybeFile::Embedded(_, p) => p,
        }
    }
    pub fn is_embedded(&self) -> bool {
        match self {
            MaybeFile::File(_) => false,
            MaybeFile::Embedded(_, _) => true,
        }
    }
}

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
