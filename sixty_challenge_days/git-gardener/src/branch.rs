use anyhow::Result;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use chrono::{Days, Months};
use colored::Colorize;
use git2::Time;

use crate::arg::{GitGardenerArgs, PeriodArgs, StalenessDate};
use crate::GitGardener;

#[derive(PartialEq, Eq, Debug)]
pub struct Branch {
    pub number: u64,
    pub period: PeriodArgs,
}

impl Branch {
    pub fn load_from_args(args: &Option<StalenessDate>) -> Option<Self> {
        args.as_ref().and_then(|steleness| match steleness {
            StalenessDate::Steleness { number, period } => Some(Self {
                number: *number,
                period: period.clone(),
            }),
        })
    }

    pub fn delete_steleness_branches(
        &self,
        args: &GitGardenerArgs,
        gardener: &GitGardener,
    ) -> Result<()> {
        let period_of = match self.period {
            PeriodArgs::Days => Local::now().checked_sub_days(Days::new(self.number)),
            PeriodArgs::Months => {
                Local::now().checked_sub_months(Months::new(self.number.try_into()?))
            }
        };

        let repository = gardener
            .repository
            .as_ref()
            .expect("Repository has already be set");

        let branches = repository.branches(Some(git2::BranchType::Local))?;
        let branches = branches.filter_map(|branch| branch.ok());

        for (mut branch, _) in branches {
            if let Ok(commit) = branch.get().peel_to_commit() {
                let time = self.get_commit_date(commit.time());
                if period_of > time {
                    if args.dry_run {
                        if let Some(name) =
                            branch.get().name().and_then(|name| name.split("/").last())
                        {
                            if name != args.main_branch {
                                println!("This branch will be deleted {}", name.red());
                            }
                        }
                    } else {
                        branch.delete()?;
                    }
                }
            }
        }
        Ok(())
    }

    fn get_commit_date(&self, time: Time) -> Option<DateTime<Local>> {
        let hour = 60;
        FixedOffset::east_opt(time.offset_minutes() * hour).map(|offset| {
            let datetime = NaiveDateTime::from_timestamp_opt(time.seconds(), 0)?;
            let datetime = DateTime::<Local>::from_naive_utc_and_offset(datetime, offset);

            Some(datetime)
        })?
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use git2::Repository;
    use std::path::PathBuf;

    #[test]
    fn test_load_from_args() {
        let args: Option<StalenessDate> = None;
        let branch = Branch::load_from_args(&args);
        assert_eq!(branch, None);

        let args: Option<StalenessDate> = Some(StalenessDate::Steleness {
            number: 5,
            period: PeriodArgs::Days,
        });
        let branch = Branch::load_from_args(&args);
        assert_eq!(
            branch,
            Some(Branch {
                number: 5,
                period: PeriodArgs::Days,
            })
        );
    }

    #[test]
    fn test_delete_steleness_branches() -> Result<()> {
        let repo_path = PathBuf::from("/tmp/test_repo");
        let repo = Repository::init(&repo_path)?;
        let gardener = GitGardener {
            repository: Some(repo),
            branch: None,
        };

        let branch = Branch {
            number: 7,
            period: PeriodArgs::Months,
        };

        let args = GitGardenerArgs {
            dry_run: true,
            main_branch: "master".to_string(),
            git_repository: repo_path.clone(),
            command: None,
        };
        branch.delete_steleness_branches(&args, &gardener)?;

        let args = GitGardenerArgs {
            dry_run: false,
            main_branch: "master".to_string(),
            git_repository: repo_path,
            command: None,
        };

        branch.delete_steleness_branches(&args, &gardener)?;

        Ok(())
    }

    #[test]
    fn test_get_commit_date() {
        let branch = Branch {
            number: 7,
            period: PeriodArgs::Months,
        };

        let time = Time::new(1627639200, 0);

        let commit_date = branch.get_commit_date(time);
        assert_eq!(commit_date.is_some(), true);
    }
}
