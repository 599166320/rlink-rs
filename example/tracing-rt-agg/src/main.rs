#[macro_use]
extern crate rlink_derive;
#[macro_use]
extern crate async_trait;
mod app;
#[tokio::main]
async fn main() {
    rlink::core::env::execute(app::TracingRtStreamApp {}).await;
}
