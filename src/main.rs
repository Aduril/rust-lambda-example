use lambda_http::handler;
use lambda_http::Context;
use lambda_http::{IntoResponse, Request};
use lambda_runtime::{self, Error};
use jemallocator;

async fn apply(mut req: Request, _c: Context) -> Result<impl IntoResponse, Error> {
    let payload = req.body_mut();
    match payload {
        _ => {
            println!("hello!");
            Ok("Hello World")
        },
    }
}

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler(apply);
    lambda_runtime::run(func).await?;
    Ok(())
}
