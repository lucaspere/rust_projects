use std::{
    ffi::OsString,
    io::{self, ErrorKind},
    path::PathBuf,
};

use clap::{
    arg,
    builder::{OsStringValueParser, PossibleValue, TypedValueParser},
    Parser, Subcommand,
};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(name = "Git Gardener")]
#[command(
    version,
    arg(clap::Arg::new("dummy")
        .value_parser([PossibleValue::new("git-gardener")])
        .required(false)
        .hide(true))
)]
#[command(about = "Tidies up your Git Branches", long_about = None)]
pub struct GitGardenerArgs {
    #[arg(
        short,
        long,
        help = "The principal branch of repository",
        default_value = "main"
    )]
    pub main_branch: String,

    #[arg(short, long, help = "Show what would be deleted")]
    pub dry_run: bool,

    #[arg(short, long, env, default_value = ".", value_parser = OsStringValueParser::new().try_map(parse_path) , help = "Path of the Git Repository to use")]
    pub git_repository: PathBuf,

    #[command(subcommand)]
    pub command: Option<StalenessDate>,
}

#[derive(clap::ValueEnum, Default, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PeriodArgs {
    Days,
    #[default]
    Months,
}

#[derive(Subcommand, Debug)]
pub enum StalenessDate {
    /// Set the staleness cutoff date of your branch steleness.
    ///
    /// This command will use the values to determine the period to delete a branch
    Steleness {
        #[arg(
            short,
            long,
            required = true,
            help = r"Number of PERIOD to use as a reference"
        )]
        number: u64,

        #[arg(
            short,
            long,
            help = "Period of a duration to use as reference",
            value_enum,
            default_value_t
        )]
        period: PeriodArgs,
    },
}

/// Helper function to parse and return the absolute path
///
/// Source: [shuttle-args](https://github.com/shuttle-hq/shuttle/blob/05a37656b272255cd665ba58fa411bb52abc86d0/cargo-shuttle/src/args.rs#L411)
fn parse_path(path: OsString) -> Result<PathBuf, io::Error> {
    dunce::canonicalize(&path).map_err(|e| {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!("could not turn {path:?} into a real path: {e}"),
        )
    })
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use crate::arg::GitGardenerArgs;

    #[test]
    fn test_git_gardener_args() {
        GitGardenerArgs::command().debug_assert()
    }
}
