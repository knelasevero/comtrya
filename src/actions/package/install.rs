use crate::actions::command::CommandAction;
use crate::actions::{Action, ActionError, ActionResult};
use crate::manifest::Manifest;
use std::ops::Deref;
use tera::Context;

use super::Package;
use super::PackageVariant;

pub type PackageInstall = Package;

impl CommandAction for PackageInstall {}

impl Action for PackageInstall {
    fn run(
        self: &Self,
        _manifest: &Manifest,
        _context: &Context,
    ) -> Result<ActionResult, ActionError> {
        let variant: PackageVariant = self.into();
        let box_provider = variant.provider.clone().get_provider();
        let provider = box_provider.deref();

        // If the provider isn't available, see if we can bootstrap it
        if false == provider.available() {
            match provider.bootstrap() {
                Ok(_) => {}
                Err(_) => {
                    return Err(ActionError {
                        message: String::from("Provider unavailable"),
                    });
                }
            }
        }

        match provider.install(variant.packages()) {
            Ok(_) => {
                println!("Installed");
                ()
            }
            Err(e) => {
                return Err(e);
            }
        }

        Ok(ActionResult {
            message: String::from("Done"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::Actions;

    #[test]
    fn it_can_be_deserialized() {
        let yaml = r#"
- action: package.install
  name: comtrya

- action: package.install
  list:
    - comtrya
"#;

        let mut actions: Vec<Actions> = serde_yaml::from_str(yaml).unwrap();

        match actions.pop() {
            Some(Actions::PackageInstall(package_install)) => {
                assert_eq!(vec!["comtrya"], package_install.list);

                ()
            }
            _ => {
                assert!(
                    false,
                    "PackageInstall didn't deserialize to the correct type"
                );

                ()
            }
        };

        match actions.pop() {
            Some(Actions::PackageInstall(package_install)) => {
                assert_eq!("comtrya", package_install.name.clone().unwrap());
                ()
            }
            _ => {
                assert!(
                    false,
                    "PackageInstall didn't deserialize to the correct type"
                );

                ()
            }
        };

        ()
    }
}
