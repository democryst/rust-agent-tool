use crate::ports::ActionVerifier;
use async_trait::async_trait;
use std::process::Command;

pub struct CargoVerifier;

#[async_trait]
impl ActionVerifier for CargoVerifier {
    async fn verify_system_state(&self) -> Result<bool, String> {
        let output = Command::new("cargo")
            .arg("check")
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    Ok(true)
                } else {
                    let err = String::from_utf8_lossy(&out.stderr).to_string();
                    Err(err)
                }
            }
            Err(e) => Err(format!("Failed to execute cargo check: {}", e)),
        }
    }
}
