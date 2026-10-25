use std::process::Command;

pub fn gh_mutation(mutation: &str) -> Result<(), String> {
    let output = Command::new("gh")
        .args([
            "api",
            "graphql",
            "-f",
            format!("query={}", mutation).as_str(),
        ])
        .output()
        .map_err(|e| format!("Failed to run gh api graphql: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", stderr);
        return Err(stderr.trim().to_string());
    }

    Ok(())
}
