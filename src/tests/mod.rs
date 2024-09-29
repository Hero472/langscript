#[cfg(test)]
mod tests {
    use core::str;
    use std::process::Command;


    #[test]
    fn interpret_block(){
        
        let output: std::process::Output = Command::new("cargo")
            .arg("run")
            .arg(r"src\tests\cases\block.lss")
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
}