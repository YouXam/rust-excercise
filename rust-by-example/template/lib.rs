



#[allow(dead_code)]
const EXCEPT_OUTPUT: &str = "Hello World!";
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use std::process::Command;
    #[test]
    #[ignore]
    fn test_output() {
        let output = Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8(output.stdout).unwrap();

        assert_eq!(output_str.trim(), EXCEPT_OUTPUT.trim());
    }
}
