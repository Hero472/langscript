#[cfg(test)]
mod tests {
    use std::{fs::{read_dir, read_to_string, DirEntry}, process::Command};


    #[test]
    fn execute_tests() {
        let cases: std::fs::ReadDir = read_dir(r"C:\Users\herod\Workplace\Rust\Training\langscript\src\tests\cases").unwrap();

        let mut errors: Vec<String> = vec![];
        for case in cases {
            let case: DirEntry = case.unwrap();
            let name: String = case.path().display().to_string();
            if name.contains("~") {
                continue;
            }

            match run_test(case) {
                Ok(_) => (),
                Err(msg) => {
                    errors.push(msg);
                    break;
                }
            }
        }

        if errors.len() > 0 {
            panic!("Errors:\n{}", errors.join("\n\n"));
        }

    }

    fn run_test(file: DirEntry) -> Result<(), String> {
        let contents: String = read_to_string(file.path()).unwrap();
        let lines: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

        let mut test_code: Vec<&str> = vec![];

        let mut idx: Option<usize> = None;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("--- TEST") {
                continue;
            }
            if line.starts_with("--- EXPECTED") {
                idx = Some(i);
                break;
            }
            test_code.push(line.clone());
        }

        println!("{:?}", test_code);

        let idx: usize = idx.expect(&format!(
            "{:#?}: No expected section in test case definition",
            file.file_name()
        ));

        let mut expected_output: Vec<&str> = vec![];

        for line in &lines[idx + 1..] {
            if line.len() > 0 {
                expected_output.push(*line);
            }
        }

        let input: String = test_code.join("\n");

        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg("e")
            .arg(input)
            .output()
            .unwrap();

        let lines: Vec<&str> = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        if !(lines.len() == expected_output.len() || lines.len() == expected_output.len() + 1) {
            return Err(format!(
                "{:#?}: output length does not match expected output: {} != {}\nFull output:\n{}",
                file.file_name(),
                lines.len(),
                expected_output.len(),
                lines.join("\n")
            ));
        }

        for (i, expected) in expected_output.iter().enumerate() {
            if lines[i] != (*expected).trim() {
                return Err(format!(
                    "{:#?}: {} != {}\nFull output:\n{}",
                    file.file_name(),
                    lines[i],
                    expected,
                    lines.join("\n")
                ));
            }
        }

        Ok(())
    }
}