use secrets_manager_macro::build_secrets_struct;

#[build_secrets_struct]
pub enum DoesNotWorkForEnum {}

fn main() {}