use derive_more::From;
use lib_core::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),
    // -- Modules
    #[from]
    Model(model::Error),
}

// Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
