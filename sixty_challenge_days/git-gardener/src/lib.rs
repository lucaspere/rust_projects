mod arg;
mod branch;

use anyhow::{Ok, Result};
use arg::{GitGardenerArgs, StalenessDate};
use branch::Branch;
use clap::{CommandFactory, FromArgMatches};
use git2::Repository;

pub struct GitGardener {
    pub repository: Option<Repository>,
    pub branch: Option<Branch>,
}

impl GitGardener {
    pub fn new() -> Self {
        Self {
            repository: None,
            branch: None,
        }
    }
    pub fn parse_args_and_run(mut self) -> Result<()> {
        let matches = GitGardenerArgs::command().get_matches();
        let args = GitGardenerArgs::from_arg_matches(&matches)
            .expect("args to already to parsed successfully");

        self.repository = Some(git2::Repository::open(&args.git_repository)?);

        if matches!(args.command, Some(StalenessDate::Steleness { .. })) {
            self.branch = Branch::load_from_args(&args.command);
        }
        self.run(args)?;

        Ok(())
    }

    pub fn run(&self, args: GitGardenerArgs) -> Result<()> {
        self.branch
            .as_ref()
            .and_then(|branch| branch.delete_steleness_branches(&args, &self).ok());
        Ok(())
    }
}
