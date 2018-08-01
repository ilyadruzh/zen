//! Basic clap factory implementation.
//! This module collect all basic `CommandExtension` that
//! we can use in `employees` bootstrapping process.
//!
use exonum::blockchain::Service;
use exonum::helpers::fabric::{Argument, CommandExtension, CommandName, Context, keys,
                              ServiceFactory};
use exonum::node::NodeConfig;
use failure;
use service::EmployeesService;
use std::collections::BTreeMap;
use toml::Value;

/// Employees configuration that should be saved into the file
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmployeesServiceConfig {
    /// Superuser public key.
    pub superuser_pkey: String,
}

struct GenerateCommonConfig;

impl CommandExtension for GenerateCommonConfig {
    fn args(&self) -> Vec<Argument> {
        vec![
            Argument::new_named(
                "EMPLOYEES_SUPERUSER_PUB_KEY",
                true,
                "Public key of the superuser.",
                None,
                "superuser-pkey",
                false,
            ),
        ]
    }

    fn execute(&self, mut context: Context) -> Result<Context, failure::Error> {
        let pkey = context.arg::<String>("EMPLOYEES_SUPERUSER_PUB_KEY").ok();

        let mut values: BTreeMap<String, Value> = context.get(keys::SERVICES_CONFIG).expect(
            "Expected services_config \
             in context.",
        );

        values.extend(
            vec![
                (
                    "superuser_pub_key".to_owned(),
                    Value::try_from(pkey).unwrap(),
                ),
            ].into_iter(),
        );

        context.set(keys::SERVICES_CONFIG, values);
        Ok(context)
    }
}

struct Finalize;

impl CommandExtension for Finalize {
    fn args(&self) -> Vec<Argument> {
        vec![]
    }

    fn execute(&self, mut context: Context) -> Result<Context, failure::Error> {
        let mut node_config: NodeConfig = context.get(keys::NODE_CONFIG).unwrap();
        let common_config = context.get(keys::COMMON_CONFIG).unwrap();

        // Global config section
        let superuser_pkey: String = common_config
            .services_config
            .get("superuser_pub_key")
            .expect("Superuser public key is not found")
            .clone()
            .try_into()?;

        node_config.services_configs.insert(
            "employees_service".to_owned(),
            Value::try_from(EmployeesServiceConfig {
                superuser_pkey,
            }).expect("Could not serialize employees service config"),
        );

        context.set(keys::NODE_CONFIG, node_config);
        Ok(context)
    }
}

/// An employees service creator for the `NodeBuilder`.
#[derive(Debug)]
pub struct RoleSystemServiceFactory;

impl ServiceFactory for RoleSystemServiceFactory {
    #[allow(unused_variables)]
    fn command(&mut self, command: CommandName) -> Option<Box<CommandExtension>> {
        use exonum::helpers::fabric;
        Some(match command {
            v if v == fabric::GenerateCommonConfig::name() => Box::new(GenerateCommonConfig),
            v if v == fabric::Finalize::name() => Box::new(Finalize),

            _ => return None,
        })
    }

    fn make_service(&mut self, run_context: &Context) -> Box<Service> {
        let employees_service_config: EmployeesServiceConfig =
            run_context.get(keys::NODE_CONFIG).unwrap().services_configs["employees_service"]
                .clone()
                .try_into()
                .unwrap();

        Box::new(EmployeesService::new(
            employees_service_config.superuser_pkey
        ))
    }
}
