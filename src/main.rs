use warp::{Filter, Reply};
use warp::http::StatusCode;

type AppResult<T> = anyhow::Result<T>;
type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

#[tokio::main]
async fn main() {
	let route1 = warp::path!("api" / "v1" / "customers")
		.and(warp::post())
		.and(warp::body::json())
		.and_then(handler);

	let route2 = warp::path!("api" / "v1" / "customers" / String)
		.and(warp::get())
		.map(|id: String| warp::reply::with_status(format!("hello customers: {}", id), StatusCode::OK));

	let r = route1.or(route2);

	warp::serve(r).run(([127, 0, 0, 1], 3031)).await;
}

async fn handler(req: AuthRequest) -> WebResult<impl Reply> {
	Ok(warp::reply::with_status(
		format!("{}, {}", req.username, req.password),
		StatusCode::OK,
	))
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequest {
	pub username: String,
	pub password: String,
}
