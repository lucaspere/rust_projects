use anyhow::Result;
use git_gardener::GitGardener;

fn main() -> Result<()> {
    let git_gardener = GitGardener::new();
    git_gardener.parse_args_and_run()
}
