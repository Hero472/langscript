#[cfg(test)]
mod tests {
    use core::str;
    use std::process::Command;


    #[test]
    fn interpret_block(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg("./src/tests/cases/block.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "4");
        assert_eq!(lines[1], "3");
    }

    #[test]
    fn interpret_while(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\while.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "2");
        assert_eq!(lines[1], "1");
    }

    #[test]
    fn interpret_while_break(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\whilebreak.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "2");
    }


    #[test]
    fn interpret_math(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\math.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();

        assert_eq!(lines.len(), 11);
        assert_eq!(lines[0], "10");
        assert_eq!(lines[1], "90");
        assert_eq!(lines[2], "720");
        assert_eq!(lines[3], "5040");
        assert_eq!(lines[4], "30240");
        assert_eq!(lines[5], "151200");
        assert_eq!(lines[6], "604800");
        assert_eq!(lines[7], "1814400");
        assert_eq!(lines[8], "3628800");
        assert_eq!(lines[9], "3628800");

    }

    #[test]
    fn interpret_for(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\for.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();

        let mut fibo: Vec<i32> = vec![];
        let mut a: i32 = 0;
        let mut b: i32 = 1;
        let mut tmp: i32;
        for _ in 0..21 {
            fibo.push(a);
            tmp = b;
            b = a + b;
            a = tmp;
        }

        assert_eq!(lines.len(), fibo.len() + 1);
        
        for i in 0..fibo.len() {
            assert_eq!(lines[i], fibo[i].to_string())
        }
    }

    #[test]
    fn interpret_fun_def() {
        let output =
            Command::new("cargo")
            .arg("run")
            .arg("./src/tests/cases/fundef.lss")
            .output()
            .unwrap();
        let lines: Vec<&str> = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        assert_eq!(lines.len(), 4, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "3");
    }


    #[test]
    fn interpret_fun_local_env(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\fun_local_env.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        println!("{:?}", lines);
        assert_eq!(lines.len(), 2, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "3");
    }

    #[test]
    fn interpret_fun_return(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\fun_return.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        println!("{:?}", lines);
        assert_eq!(lines.len(), 2, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "5");
    }

    #[test]
    fn interpret_fun_scope(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\scope_test.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        println!("{:?}", lines);
        assert_eq!(lines.len(), 2, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "6");
    }

    #[test]
    fn interpret_fun_return_null(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\fun_return_null.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        println!("{:?}", lines);
        assert_eq!(lines.len(), 3, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "");
    }

    #[test]
    fn interpret_fun_cond_return(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\fun_cond_return.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        println!("{:?}", lines);
        assert_eq!(lines.len(), 5, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "3");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "1");
        assert_eq!(lines[3], "0");
    }

    #[test]
    fn interpret_fun_nested(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\fun_nested.lss")
            .output()
            .unwrap();

        let lines: Vec<&str> = str::from_utf8(output.stdout.as_slice()).unwrap().split("\n").collect::<Vec<&str>>();
        println!("{:?}", lines);
        assert_eq!(lines.len(), 3, "Output: '{}'", lines.join("\n"));
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "7");
    }

}