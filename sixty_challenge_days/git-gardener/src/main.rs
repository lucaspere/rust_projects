use std::path::PathBuf;

/// Tasks:
/// [x] - Find a way to subtract two dates
/// [x] - Learn how remove a branch for in a period;
/// [x] - Only delete a branch where is fully contained in another.
/// [x] - use clap to define de range of period to delete branch.
///
use chrono::{DateTime, Days, FixedOffset, Local, Months, NaiveDateTime};
use clap::{Parser, Subcommand};
use colored::Colorize;
use git2::{Error, Time};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(name = "Git Garderne")]
#[command(version)]
#[command(about = "Tidies up your Git Branches", long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        help = "The principal branch of repository",
        default_value = "main"
    )]
    main_branch: String,

    #[arg(short, long, help = "show what would be deleted")]
    dry_run: bool,

    #[arg(short, long, env, help = "Path of the Git Repository to use")]
    git_repository: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<StalenessDate>,
}

#[derive(clap::ValueEnum, Default, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Period {
    Days,
    #[default]
    Months,
}

#[derive(Subcommand, Debug)]
enum StalenessDate {
    /// Set the duration of your branch steleness.
    ///
    /// This command will use the values to determine the period to delete a branch
    Steleness {
        #[arg(
            short,
            long,
            required = true,
            help = r"Number of PERIOD to use as a reference"
        )]
        number: u8,

        #[arg(
            short,
            long,
            help = "Period of a duration to use as reference",
            value_enum,
            default_value_t
        )]
        period: Period,
    },
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    println!("{:?}", cli);
    let repository = cli.git_repository.map_or_else(
        || git2::Repository::open(""),
        |path| git2::Repository::open(path),
    )?;

    let ref_period = cli
        .command
        .map(|StalenessDate::Steleness { number, period }| match period {
            Period::Days => Local::now().checked_sub_days(Days::new(number.into())),
            Period::Months => Local::now().checked_sub_months(Months::new(number.into())),
        })
        .unwrap();

    if let Ok(branches) = repository.branches(Some(git2::BranchType::Local)) {
        let branches = branches.filter_map(|branch| branch.ok());

        for (mut branch, _) in branches {
            if cli.dry_run {
                if let Ok(commit) = branch.get().peel_to_commit() {
                    let time = get_commit_date(commit.time());
                    if ref_period > time {
                        if cli.dry_run {
                            if let Some(name) =
                                branch.get().name().and_then(|name| name.split("/").last())
                            {
                                if name != cli.main_branch {
                                    println!("This branch will be deleted {}", name.red());
                                }
                            }
                        } else {
                            branch.delete()?
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn get_commit_date(time: Time) -> Option<DateTime<Local>> {
    let hour = 60;
    FixedOffset::east_opt(time.offset_minutes() * hour).map(|offset| {
        let datetime = NaiveDateTime::from_timestamp_opt(time.seconds(), 0)?;
        let datetime = DateTime::<Local>::from_naive_utc_and_offset(datetime, offset);

        Some(datetime)
    })?
}

// #[cfg(test)]
// mod tests {
//     use core::time;

//     use chrono::Local;

//     #[test]
//     fn should_subtract_dates() {
//         let date1 = Local::now();

//         std::thread::sleep(time::Duration::from_secs(4));
//         let date2 = Local::now();

//         let diff = subDates(date1, date2);
//     }
// }
