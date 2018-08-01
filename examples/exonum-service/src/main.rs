// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate exonum;
extern crate exonum_configuration;
extern crate exonum_employees as employees;

use employees::cmd::EmployeesServiceFactory;
use exonum::helpers;
use exonum::helpers::fabric::NodeBuilder;
use exonum_configuration as configuration;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");

    exonum::crypto::init();
    helpers::init_logger().unwrap();

    let node = NodeBuilder::new()
        .with_service(Box::new(configuration::ServiceFactory))
        .with_service(Box::new(EmployeesServiceFactory));

    node.run();
}
