



#[allow(dead_code)]
const EXCEPT_OUTPUT: &str = "Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }";
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use std::process::Command;
    #[test]
    // #[ignore]
    fn test_output() {
        let output = Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8(output.stdout).unwrap();

        assert_eq!(output_str.trim(), EXCEPT_OUTPUT.trim());
    }
}
