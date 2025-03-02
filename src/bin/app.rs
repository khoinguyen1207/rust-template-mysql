use rust_template_mysql::app;
use std::io;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> io::Result<()> {
    app::create_app().await
}
