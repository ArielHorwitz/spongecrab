#[cfg(test)]
mod integration_tests {
    use spongecrab::CliBuilder;

    #[test]
    fn parse() {
        let raw_args = [
            "app", "-p", "pos1", "-O", "opt1", "--", "valpos1", "--opt1", "valopt1",
        ];
        let output = CliBuilder::new(&raw_args).parse().expect("parse");
        assert!(output.contains("pos1='valpos1'"));
        assert!(output.contains("opt1='valopt1'"));
    }

    #[test]
    fn generate() {
        let generated = CliBuilder::new(&["app", "--generate"])
            .parse()
            .expect("generate")
            .replace("spongecrab ", "cargo run -qr -- ");
        let cli_name = "neo";
        let cli_greetings = "greetings";
        let cli_notice = "follow the white rabbit";
        let cli_polite = "1";
        let generated = format!(
            "
            set -e
            {generated}
            echo -n parsed.
            echo -n name $name.
            [[ $name = \"{cli_name}\" ]]
            echo -n greetings $greetings.
            [[ $greetings = \"{cli_greetings}\" ]]
            echo -n notice $notice.
            [[ $notice = \"{cli_notice}\" ]]
            echo -n polite $polite.
            [[ $polite = \"{cli_polite}\" ]]
        "
        );
        std::fs::write("/tmp/gentest.sh", generated).expect("write");
        let output = std::process::Command::new("bash")
            .arg("/tmp/gentest.sh")
            .args([cli_name, cli_greetings, "-n", cli_notice, "-p"])
            .output()
            .expect("generated test script success");
        dbg!(&output);
        assert!(output.status.success());
    }
}
