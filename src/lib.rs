//! glfn - describe me / TODO

// TODO: #[deny(missing_docs)] or whatever

use roxmltree::Document;
use std::{fmt, error, io};
use crate::Error::Registry;

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
    ///
    /// Generally this should never be emitted unless Khronos made a mistake in an update.
    Registry(RegistryError),

    /// Failed to parse XML file.
    ///
    /// Note that this only refers to XML validity, not the registry data being correct.
    /// Generally this should never be emitted unless Khronos made a mistake in an update.
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
pub enum API {
    GL,
    // ...
}

pub struct Generator {
    api: API,
}

impl Generator {
    pub fn new(api: API) -> Self {
        Self { api }
    }

    pub fn generate(&self, out: &mut impl io::Write) -> Result<(), Error> {
        let _ = out; // we'll write here eventually

        let doc = Document::parse(api_to_xml(self.api))?;
        let registry = match doc.descendants().find(|node| node.has_tag_name("registry")) {
            Some(registry) => registry,
            None => Err(RegistryError::MissingRoot.into()),
        };

        Ok(())
    }
}

fn api_to_xml(api: API) -> &'static str {
    let xml = match api {
        API::GL => xml::GL,
    };
    xml.trim_start_matches('\u{feff}')
}
