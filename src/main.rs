mod handlers;

use std::env;
use std::net::TcpListener;

use awesome_blog::start_blog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	let port = env::var("PORT")
		.unwrap_or_else(|_| "8080".to_string())
		.parse()
		.expect("Invalid port number");

	let listener = TcpListener::bind(("0.0.0.0", port))?;
	start_blog(listener)?.await?;
	Ok(())
}
