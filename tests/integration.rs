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
        let args = [
            ("arg1", "neo"),
            ("arg2", "greetings"),
            ("option", "follow the white rabbit"),
            ("flag", "1"),
        ];
        let checks = check_args_code(&args);
        let args = [args[0].1, args[1].1, "-o", args[2].1, "-f"];
        let script = format!("set -e\n{generated}\n{checks}");
        std::fs::write("/tmp/test_generate.sh", script).expect("write");
        let output = std::process::Command::new("bash")
            .arg("/tmp/test_generate.sh")
            .args(args)
            .output()
            .expect("generated test script success");
        dbg!(&output);
        assert!(output.status.success());
    }

    #[test]
    fn example() {
        let generated = CliBuilder::new(&["app", "--example"])
            .parse()
            .expect("example")
            .replace("spongecrab ", "cargo run -qr -- ");
        let args = [
            ("name", "neo"),
            ("greetings", "greetings"),
            ("notice", "follow the white rabbit"),
            ("polite", "1"),
        ];
        let checks = check_args_code(&args);
        let args = [args[0].1, args[1].1, "-n", args[2].1, "-p"];
        let script = format!("set -e\n{generated}\n{checks}");
        std::fs::write("/tmp/test_example.sh", script).expect("write");
        let output = std::process::Command::new("bash")
            .arg("/tmp/test_example.sh")
            .args(args)
            .output()
            .expect("generated test script success");
        dbg!(&output);
        assert!(output.status.success());
    }

    fn check_args_code(args: &[(&str, &str)]) -> String {
        args.iter()
            .map(|(n, v)| format!("echo -n {n}:${n},\n[[ ${n} = \"{v}\" ]]"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
