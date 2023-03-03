use clap::{ArgMatches, Command};
use git2::{Repository, Worktree};

fn build_cli() -> Command {
    let cmd = Command::new("wt")
        .arg_required_else_help(true)
        .subcommand(Command::new("list").about("List worktrees"))
        .subcommand(Command::new("add").about("Add a worktree"));
    return cmd;
}

enum WorktreesError {
    GitError(git2::Error),
    ApplicationError,
}

fn get_worktrees() -> Result<(Repository, Vec<String>), WorktreesError> {
    let cwd = ".";
    if let Ok(repo) = Repository::discover(cwd) {
        let trees = repo
            .worktrees()
            .map(|all_trees| {
                return all_trees
                    .iter()
                    .filter_map(|i| Some(String::from(i?)))
                    .collect::<Vec<String>>();
            })
            .map_err(|inner_err| WorktreesError::GitError(inner_err));
        Ok((repo, trees?))
    } else {
        Err(WorktreesError::ApplicationError)
    }
}

fn print_worktrees() -> () {
    match get_worktrees() {
        Ok((repo, tree_names)) => {
            for tree_name in tree_names {
                let tree: Worktree = repo
                    .find_worktree(tree_name.as_str())
                    .expect("This isname not reachable");
                let (path, name) = (tree.path().display(), String::from(tree.name().unwrap()));
                let git_ref = Repository::open(tree.path());
                println!("{}\t{}", path, name)
            }
        }
        _ => println!("Found no worktrees"),
    };
}

fn add_worktree() -> () {
    todo!()
}

fn main() {
    let matches: ArgMatches = build_cli().get_matches();
    match matches.subcommand() {
        Some(("list", _)) => print_worktrees(),
        Some(("add", _)) => add_worktree(),
        _ => {}
    }
}

#[cfg(test)]
mod test {}
