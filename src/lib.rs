//! glfn - describe me / TODO

// TODO: #[deny(missing_docs)] or whatever

use roxmltree::Document;
use std::{fmt, error, io};

/// Re-exported dependencies.
pub mod deps {
    pub use roxmltree;
}

// the big xml registry constants
#[doc(hidden)]
pub mod xml {
    pub const GL: &str = include_str!("../khronos/registry/gl.xml");
}

// error & result types
#[derive(Debug)]
pub enum Error {
    /// Writing to output stream resulted in [io::Error].
    ///
    /// [io::Error]: std::io::Error
    IO(io::Error),

    /// Invalid XML registry structure encountered.
    /// This error is not emitted on official Khronos inputs.
    Registry(RegistryError),

    /// Failed to parse XML file.
    ///
    /// Note that this only refers to XML validity, not the registry data being correct.
    /// This error is not emitted on official Khronos inputs.
    XML(crate::deps::roxmltree::Error),
}
#[derive(Debug)]
pub enum RegistryError {
    /// The root <registry> tag was not found.
    MissingRoot,
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(e) => write!(f, "io error: {}", e),
            Self::Registry(e) => write!(f, "registry structure error: {}", e),
            Self::XML(e) => write!(f, "xml parsing error: {}", e),
        }
    }
}
impl error::Error for RegistryError {}
impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingRoot => write!(f, "missing root <registry> tag"),
        }
    }
}
macro_rules! error_from_err {
    ($variant: ident, $ty: ty) => {
        impl From<$ty> for Error {
            fn from(err: $ty) -> Self {
                Self::$variant(err)
            }
        }
    };
}
error_from_err!(IO, io::Error);
error_from_err!(Registry, RegistryError);
error_from_err!(XML, roxmltree::Error);

#[derive(Copy, Clone)]
pub enum API<'src> {
    GL,

    Custom(&'src str),
}

pub struct Generator<'registry> {
    api: API<'registry>,
}

impl<'registry> Generator<'registry> {
    pub fn new(api: API<'registry>) -> Self {
        Self { api }
    }

    pub fn generate(&self) -> Result<Registry, Error> {
        let doc = Document::parse(api_to_xml(self.api))?;
        Ok(Registry::new(doc)?)
    }
}

pub struct Registry {

}

impl Registry {
    fn new(xml: roxmltree::Document) -> Result<Self, RegistryError> {
        let root = match xml.descendants().find(|node| node.has_tag_name("registry")) {
            Some(registry) => registry,
            None => return Err(RegistryError::MissingRoot),
        };
        Ok(Registry {})
    }
}

fn api_to_xml(api: API) -> &str {
    let xml = match api {
        API::GL => xml::GL,
        API::Custom(custom) => custom,
    };
    trim_bom(xml)
}

fn trim_bom(s: &str) -> &str {
    s.trim_start_matches('\u{feff}')
}

#[cfg(test)]
mod tests {
    use super::xml;
    use roxmltree::Document;

    #[test]
    fn xml_files_parse() {
        let _ = Document::parse(xml::GL).unwrap();
    }
}
