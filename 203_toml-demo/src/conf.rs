pub(crate) mod basic_conf;
pub(crate) mod error;
pub(crate) mod poem_conf;

use super::*;
use std::borrow::Cow;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;

pub use self::basic_conf::BasicConfig;
pub use self::error::ConfigError;
pub use self::poem_conf::PoemConfig;

use crate::environment::{Environment, Environment::*};
use std::collections::HashMap;

pub const CONFIG_FILENAME: &str = "conf/Poem.toml";
pub type Result<T> = ::std::result::Result<T, ConfigError>;
