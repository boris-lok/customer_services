use crate::core::config::Config;

#[derive(Debug, Clone)]
pub struct Environment {
	pub config: Config,
}

impl Environment {
	pub fn new(config: Config) -> Self {
		Self { config }
	}
}