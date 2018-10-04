extern crate exonum;
extern crate exonum_configuration;
extern crate zen_local_database;
extern crate zen_rolesystem;

use exonum::helpers;
use exonum::helpers::fabric::NodeBuilder;
use exonum_configuration as configuration;
use zen_local_database::cmd::ZenLocalDBServiceFactory;
use zen_rolesystem::cmd::RoleSystemServiceFactory;

use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");

    exonum::crypto::init();
    helpers::init_logger().unwrap();

    let node = NodeBuilder::new()
        .with_service(Box::new(configuration::ServiceFactory))
        .with_service(Box::new(ZenLocalDBServiceFactory))
        .with_service(Box::new(RoleSystemServiceFactory));

    node.run();
}
