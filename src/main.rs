use std::{
    error::Error,
    fmt::{Debug, Display},
    io::{self, Write},
    path::Path,
};

use clap::{ArgMatches, Command};
use git2::{Reference, Repository, Worktree};

fn build_cli() -> Command {
    let cmd = Command::new("wt")
        .arg_required_else_help(true)
        .subcommand(Command::new("list").about("List worktrees"))
        .subcommand(Command::new("add").about("Add a worktree"));
    return cmd;
}

#[derive(Debug)]
enum WorktreesError {
    GitError(git2::Error),
    ApplicationError,
}

impl Display for WorktreesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for WorktreesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

struct MyWorktree<'a> {
    reference: Reference<'a>,
    path: &'a str,
    name: &'a str,
}

impl<'a> MyWorktree<'a> {
    fn print(&self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
        write!(
            writer,
            "{}\t{}\t{}\n",
            self.path,
            self.name,
            self.reference
                .name()
                .ok_or(WorktreesError::ApplicationError)?
        )?;
        Ok(())
    }
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

fn print_worktrees() -> Result<(), Box<dyn Error>> {
    match get_worktrees() {
        Ok((repo, tree_names)) => {
            for tree_name in tree_names {
                let tree: Worktree = repo.find_worktree(tree_name.as_str())?;
                let tree_repo = Repository::open_from_worktree(&tree)?;
                let structured_tree = MyWorktree {
                    reference: tree_repo.head()?,
                    path: tree
                        .path()
                        .as_os_str()
                        .to_str()
                        .ok_or(WorktreesError::ApplicationError)?,
                    name: tree.name().ok_or(WorktreesError::ApplicationError)?,
                };
                /*
                    tree.path().display(),
                    String::from(tree.name().ok_or(WorktreesError::ApplicationError)?),
                    tree_repo.head()?,
                */
                structured_tree.print(&mut io::stdout().lock())?;
            }
        }
        _ => println!("Found no worktrees"),
    };
    Ok(())
}

fn add_worktree() -> () {
    todo!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches: ArgMatches = build_cli().get_matches();
    match matches.subcommand() {
        Some(("list", _)) => print_worktrees(),
        Some(("add", _)) => Ok(()),
        _ => Ok(()),
    }
}
