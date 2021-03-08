use crate::actions::{Action, ActionError, ActionResult};
use crate::manifest::Manifest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileCopy {
    pub from: String,
    pub to: String,

    #[serde(default = "get_true")]
    pub template: bool,
}

fn get_true() -> bool {
    true
}

impl FileCopy {}

impl Action for FileCopy {
    fn run(self: &Self, _manifest: &Manifest) -> Result<ActionResult, ActionError> {
        Ok(ActionResult {
            message: String::from("Copied"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::Actions;

    #[test]
    fn it_can_be_deserialized() {
        let yaml = r#"
- action: file.copy
  from: a
  to: b
"#;

        let mut actions: Vec<Actions> = serde_yaml::from_str(yaml).unwrap();

        match actions.pop() {
            Some(Actions::FileCopy(file_copy)) => {
                assert_eq!("a", file_copy.from);
                assert_eq!("b", file_copy.to);
                ()
            }
            _ => {
                assert!(false, "FileCopy didn't deserialize to the correct type");

                ()
            }
        };

        ()
    }
}
