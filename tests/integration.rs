#[cfg(test)]
mod integration_tests {
    use spongecrab::CliBuilder;

    #[test]
    fn parse() {
        let raw_args = [
            "app", "-p", "pos1", "-O", "opt1", "--", "valpos1", "--opt1", "valopt1",
        ];
        let output = CliBuilder::new(&raw_args).parse().expect("parse");
        assert!(output.contains("pos1=valpos1"));
        assert!(output.contains("opt1=valopt1"));
    }

    #[test]
    fn generate() {
        let generated = CliBuilder::new(&["app", "--generate"])
            .parse()
            .expect("generate")
            .replace("spongecrab ", "cargo run -q -- ");
        let cli_source = "srcval";
        let cli_destination = "destval";
        let cli_backup_path = "bakval";
        let cli_verbose = "1";
        let generated = format!(
            "
            set -e
            {generated}
            [[ $source = \"{cli_source}\" ]]
            [[ $destination = \"{cli_destination}\" ]]
            [[ $backup_path = \"{cli_backup_path}\" ]]
            [[ $verbose = \"{cli_verbose}\" ]]
        "
        );
        std::fs::write("/tmp/gentest.sh", generated).expect("write");
        let output = std::process::Command::new("bash")
            .arg("/tmp/gentest.sh")
            .args([cli_source, cli_destination, "-b", cli_backup_path, "-v"])
            .output()
            .expect("generated test script success");
        assert!(output.status.success());
    }
}
