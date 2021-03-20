use super::PackageProvider;
use crate::actions::{package::PackageVariant, ActionError};
use crate::utils::command::{run_command, Command};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tracing::{debug, info, span, warn};
use which::which;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aptitude {}

impl PackageProvider for Aptitude {
    fn name(&self) -> &str {
        "Aptitude"
    }

    fn available(&self) -> bool {
        match which("apt-add-repository") {
            Ok(_) => true,
            Err(_) => {
                warn!(message = "apt-add-repository not available");
                false
            }
        }
    }

    fn bootstrap(&self) -> Result<(), crate::actions::ActionError> {
        // Apt should always be available on Debian / Ubuntu flavours.
        // Lets make sure software-properties-common is available
        // for repository management
        let span = span!(tracing::Level::INFO, "bootstrap").entered();

        run_command(Command {
            name: String::from("apt"),
            args: vec![
                String::from("install"),
                String::from("-y"),
                String::from("software-properties-common"),
                String::from("gpg"),
            ],
            require_root: true,
        })?;

        span.exit();

        Ok(())
    }

    fn has_repository(&self, _package: &PackageVariant) -> bool {
        false
    }

    fn add_repository(&self, package: &PackageVariant) -> Result<(), ActionError> {
        match std::process::Command::new("apt-add-repository")
            .env("DEBIAN_FRONTEND", "noninteractive")
            .arg("-y")
            .arg(package.repository.clone().unwrap())
            .output()
        {
            Ok(_) => {
                debug!(message = "Apt Added Repository", repository = ?package.repository.clone().unwrap());
            }
            Err(error) => {
                return Err(ActionError {
                    message: error.to_string(),
                });
            }
        }

        debug!(message = "Running Aptitude Update");

        std::process::Command::new("apt")
            .arg("update")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .unwrap();

        Ok(())
    }

    fn query(&self, package: &PackageVariant) -> Vec<String> {
        package.packages()
    }

    fn install(&self, package: &PackageVariant) -> Result<(), ActionError> {
        match std::process::Command::new("apt")
            .args(&["install", "-y"])
            .args(package.extra_args.clone())
            .args(&package.packages())
            .output()
        {
            Ok(_) => {
                info!(
                    message = "Package Installed",
                    packages = package.packages().join(",").as_str()
                );

                Ok(())
            }
            Err(error) => Err(ActionError {
                message: error.to_string(),
            }),
        }
    }
}
