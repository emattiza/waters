use clap::Command;

fn build_cli() -> Command {
    let cmd = Command::new("wt").arg_required_else_help(true);
    cmd
}

fn main() {
    let matches = build_cli().get_matches();
}
