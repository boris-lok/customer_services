#[derive(Debug, Clone)]
pub struct Config {
	pub debug: bool,
	secret_key: String,
}

impl Config {
	pub fn new() -> Self {
		let _ = dotenv::from_path("config/.env").expect("Can't find the .env file.");

		let debug = dotenv::var("DEBUG")
			.map(|x| x.parse::<bool>().unwrap_or(true))
			.unwrap_or_else(true);

		if debug {
			let _ = dotenv::from_path("config/dev.env").expect("Can't find the dev.env file.");
		} else {
			let _ = dotenv::from_path("config/prod.env").expect("Can't find the prod.env file.");
		}

		let secret_key = dotenv::var("SECRET_KEY").expect("Can't read secret_key from env.");

		Self { debug, secret_key }
	}
}