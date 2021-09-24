pub(crate) mod basic_conf;
pub(crate) mod poem_conf;
pub(crate) mod error;

use super::*;
use std::fs::{self,File};
use std::io::Read;
use std::path::{PathBuf,Path};


use toml;

pub use self::error::ConfigError;
pub use self::poem_conf::PoemConfig;
pub use self::basic_conf::BasicConfig;

use crate::environment::{Environment,Environment::*};
use std::collections::HashMap;

pub const CONFIG_FILENAME: &str = "conf/Poem.toml";
pub type Result<T> = ::std::result::Result<T, ConfigError>;