use crate::actions::ActionError;

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub require_root: bool,
}

pub fn run_command(command: Command) -> Result<(), ActionError> {
    let _command = command.clone().elevate();

    Ok(())
}

impl Command {
    fn elevate(&mut self) -> &mut Self {
        // Doesn't need elevation
        if "root" == whoami::username() {
            return self;
        }

        self.args.insert(0, self.name.clone());
        self.name = String::from("sudo");
        self
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_elevate() {
        let mut command = Command {
            name: String::from("apt"),
            args: vec![String::from("install")],
            require_root: true,
        };

        command.elevate();

        assert_eq!("sudo", command.name);
    }
}
