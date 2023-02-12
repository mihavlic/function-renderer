use crate::extensions::metadata::get_metadata;
use std::{
    collections::HashSet,
    error::Error,
    ffi::CStr,
    fmt::{Debug, Display},
    os::raw::c_char,
};

pub struct ApiLoadConfig<'a> {
    api_version: u32,
    extensions: HashSet<&'a CStr>,
}

impl<'a> ApiLoadConfig<'a> {
    pub fn new(core_version: u32) -> Self {
        Self {
            api_version: core_version,
            extensions: HashSet::new(),
        }
    }
    pub fn new_with_extensions(core_version: u32, extensions: &[&'a CStr]) -> Self {
        let mut s = Self {
            api_version: core_version,
            extensions: HashSet::new(),
        };
        for &e in extensions {
            s.add_extension(e);
        }
        s
    }
    #[cfg(feature = "surface")]
    pub fn enable_surface_extensions(
        &mut self,
        window_handle: &impl raw_window_handle::HasRawWindowHandle,
    ) {
        for &e in crate::surface::enumerate_required_extensions(window_handle).unwrap() {
            self.add_extension(e);
        }
    }
    pub fn enable_all_extensions(&mut self) {
        for meta in crate::extensions::metadata::EXTENSION_METADATA {
            self.add_extension(meta.name);
        }
    }
    pub fn get_instance_extensions_iter(&self) -> ExtensionIterator<'_> {
        ExtensionIterator {
            instance: true,
            iter: self.extensions.iter(),
        }
    }
    pub fn get_instance_extensions(&self) -> Vec<*const c_char> {
        self.get_instance_extensions_iter()
            .map(|e| e.as_ptr().cast())
            .collect()
    }
    pub fn get_device_extensions_iter(&self) -> ExtensionIterator<'_> {
        ExtensionIterator {
            instance: false,
            iter: self.extensions.iter(),
        }
    }
    pub fn get_device_extensions(&self) -> Vec<*const c_char> {
        self.get_device_extensions_iter()
            .map(|e| e.as_ptr().cast())
            .collect()
    }
    pub fn get_extensions(&self) -> &HashSet<&'a CStr> {
        &self.extensions
    }
    pub fn get_api_version(&self) -> u32 {
        self.api_version
    }
    pub fn api_version_enabled(&self, version: u32) -> bool {
        self.api_version >= version
    }
    pub fn extension_enabled(&self, name: &CStr) -> bool {
        self.extensions.contains(&name)
    }
    #[track_caller]
    pub fn add_extensions_iter<T: IntoIterator<Item = &'a CStr>>(&mut self, iter: T) {
        for extension in iter {
            self.add_extension(extension)
        }
    }
    #[track_caller]
    pub fn add_extension(&mut self, name: &'a CStr) {
        let meta = get_metadata(name).unwrap_or_else(|| {
            panic!(
                "'{}' is not a valid extension. Maybe it was not generated?",
                name.to_string_lossy()
            )
        });
        if meta.core_version > self.api_version {
            panic!(
                "'{}' requires a higher core version than is configured",
                name.to_string_lossy()
            )
        }
        self.extensions.insert(name);
    }
    /// Uses the embedded extention metadata to fill in all currently selected extensions' transient dependencies.
    #[must_use]
    pub fn fill_in_extensions(&mut self) -> Result<(), ApiLoadConfigErr> {
        let extensions = self.extensions.iter().cloned().collect::<Vec<_>>();
        self.extensions.clear();
        for e in extensions {
            self.foreach_extension_dependency(e)?;
        }
        Ok(())
    }
    fn foreach_extension_dependency(&mut self, name: &CStr) -> Result<(), ApiLoadConfigErr> {
        let extension = get_metadata(name).ok_or(ApiLoadConfigErr::ExtensionNotFound)?;

        if extension.core_version > self.api_version {
            return Err(ApiLoadConfigErr::ApiVersionMismatch);
        }

        if self.extensions.insert(extension.name) == false {
            return Ok(());
        }

        for &name in extension.requires_extensions {
            self.foreach_extension_dependency(name)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ApiLoadConfigErr {
    ApiVersionMismatch,
    ExtensionNotFound,
}

impl Display for ApiLoadConfigErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiLoadConfigErr::ApiVersionMismatch => {
                write!(f, "Not a valid extension. Maybe it was not generated?")
            }
            ApiLoadConfigErr::ExtensionNotFound => write!(
                f,
                "Extension requires a higher core version than is configured."
            ),
        }
    }
}

impl Error for ApiLoadConfigErr {}

#[derive(Clone)]
pub struct ExtensionIterator<'a> {
    instance: bool,
    iter: std::collections::hash_set::Iter<'a, &'a CStr>,
}

impl<'a> Iterator for ExtensionIterator<'a> {
    type Item = &'a CStr;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&next) = self.iter.next() {
            if let Some(metadata) = get_metadata(next) {
                if metadata.instance == self.instance {
                    return Some(next);
                }
            } else {
                continue;
            }
        }

        None
    }
}
