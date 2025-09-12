use std::process::Command;

fn main() {
    // Generate constants before building
    let output = Command::new("bash")
        .arg("scripts/generate_constants.sh")
        .current_dir(".")
        .output()
        .expect("Failed to execute generate_constants.sh");

    if !output.status.success() {
        panic!(
            "Failed to generate constants: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=../data/calendars");
    println!("cargo:rerun-if-changed=../data/resources");
}
