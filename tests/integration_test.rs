mod tests {
    use assert_cmd::Command;
    #[test]
    fn it_builds_and_shows_help_when_no_args_or_commands() {
        let mut cmd = Command::cargo_bin("waters").unwrap();
        let assert = cmd.assert();
        assert
            .failure()
            .stderr(predicates::str::contains("-h, --help"));
    }
}
