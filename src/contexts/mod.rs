use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;
use user::UserContextProvider;

/// User context provider: understands the user running the command
pub mod user;

pub trait ContextProvider {
    fn get_prefix(&self) -> String;
    fn get_contexts(&self) -> Vec<Context>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Context {
    KeyValueContext(String, String),
    ListContext(String, Vec<String>),
}

pub fn build_contexts() -> tera::Context {
    let mut contexts = tera::Context::new();

    let context_providers = vec![Box::new(UserContextProvider {})];

    context_providers.iter().for_each(|provider| {
        let mut values: BTreeMap<String, Value> = BTreeMap::new();

        provider.get_contexts().iter().for_each(|context| {
            match context {
                Context::KeyValueContext(k, v) => {
                    values.insert(k.clone(), v.clone().into());
                    ()
                }
                Context::ListContext(k, v) => {
                    values.insert(k.clone(), v.clone().into());
                    ()
                }
            }

            ()
        });

        contexts.insert(provider.get_prefix(), &values);

        ()
    });

    println!("Contexts for this execution: {:?}", contexts);

    contexts
}
