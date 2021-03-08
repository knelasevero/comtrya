pub mod copy;
use crate::actions::Action;
use crate::manifest::Manifest;
use tera::Tera;

use super::ActionError;

pub trait FileAction: Action {
    fn init(&self, manifest: &Manifest) -> Tera {
        let files_directory = manifest.root_dir.clone().unwrap().join("files");

        println!(
            "Setting up tera for {:?}",
            files_directory.join("**/*").to_str().unwrap()
        );

        match Tera::new(files_directory.join("**/*").to_str().unwrap()) {
            Ok(t) => t,
            Err(e) => panic!(
                "Failed to initialise Tera for {:?}: {:?}",
                manifest.name.clone().unwrap(),
                e
            ),
        }
    }

    fn load(&self, manifest: &Manifest, path: &String) -> Result<String, ActionError> {
        std::fs::read_to_string(manifest.root_dir.clone().unwrap().join("files").join(path))
            .map_err(|e| ActionError {
                message: e.to_string(),
            })
    }
}
