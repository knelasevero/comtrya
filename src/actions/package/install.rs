use crate::actions::command::CommandAction;
use crate::actions::package::{PackageProviders, PackageVariant};
use crate::actions::{Action, ActionError, ActionResult};
use crate::manifest::Manifest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tera::Context;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInstall {
    name: Option<String>,

    #[serde(default)]
    list: Vec<String>,

    #[serde(default)]
    provider: PackageProviders,

    #[serde(default)]
    repository: Option<String>,

    #[serde(default)]
    variants: HashMap<os_info::Type, PackageVariant>,
}

impl PackageInstall {
    fn packages(&self) -> Vec<String> {
        if self.name.is_some() {
            return vec![self.name.clone().unwrap()];
        }

        if self.list.is_empty() {
            return vec![];
        }

        return self.list.clone();
    }
}

impl CommandAction for PackageInstall {}

impl Action for PackageInstall {
    fn run(
        self: &Self,
        _manifest: &Manifest,
        _context: &Context,
    ) -> Result<ActionResult, ActionError> {
        let mut command = self.init("brew");
        let mut command = self.inherit(&mut command);

        let mut args = self.packages();
        args.insert(0, String::from("install"));

        let command = self.args(&mut command, args);

        self.execute(command)
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
