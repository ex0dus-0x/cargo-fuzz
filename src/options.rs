mod add;
mod build;
mod cmin;
mod init;
mod list;
mod run;
mod tmin;

pub use self::{add::Add, build::Build, cmin::Cmin, init::Init, list::List, run::Run, tmin::Tmin};

use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, Clone, Copy)]
pub enum Sanitizer {
    Address,
    Leak,
    Memory,
    Thread,
    None,
}

impl fmt::Display for Sanitizer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sanitizer::Address => "address",
                Sanitizer::Leak => "leak",
                Sanitizer::Memory => "memory",
                Sanitizer::Thread => "thread",
                Sanitizer::None => "",
            }
        )
    }
}

impl FromStr for Sanitizer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "address" => Ok(Sanitizer::Address),
            "leak" => Ok(Sanitizer::Leak),
            "memory" => Ok(Sanitizer::Memory),
            "thread" => Ok(Sanitizer::Thread),
            "none" => Ok(Sanitizer::None),
            _ => Err(format!("unknown sanitizer: {}", s)),
        }
    }
}

#[derive(Clone, Debug, StructOpt)]
pub struct BuildOptions {
    #[structopt(short = "O", long = "release")]
    /// Build artifacts in release mode, with optimizations
    pub release: bool,

    #[structopt(short = "a", long = "debug-assertions")]
    /// Build artifacts with debug assertions enabled (default if not -O)
    pub debug_assertions: bool,

    #[structopt(long = "no-default-features")]
    /// Build artifacts with default Cargo features disabled
    pub no_default_features: bool,

    #[structopt(
        long = "all-features",
        conflicts_with = "no-default-features",
        conflicts_with = "features"
    )]
    /// Build artifacts with all Cargo features enabled
    pub all_features: bool,

    #[structopt(long = "features")]
    /// Build artifacts with given Cargo feature enabled
    pub features: Option<String>,

    #[structopt(
        short = "s",
        long = "sanitizer",
        possible_values(&["address", "leak", "memory", "thread", "none"]),
        default_value = "address",
    )]
    /// Use a specific sanitizer
    pub sanitizer: Sanitizer,

    #[structopt(
        name = "triple",
        long = "target",
        default_value(crate::utils::default_target())
    )]
    /// Target triple of the fuzz target
    pub triple: String,

    #[structopt(short = "Z", value_name = "FLAG")]
    /// Unstable (nightly-only) flags to Cargo
    pub unstable_flags: Vec<String>,
}
