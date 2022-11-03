use actix_web::{ web, App, HttpServer };
use actix_web::middleware::Logger;
use crate::utils::utils::{Config};


mod ecomrun;
mod utils;

  
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");

    let config_proxy = Config::from_env().unwrap();
   
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::new("%a %{User-Agent}i size:%b  time:%T"))
        .app_data(web::JsonConfig::default().limit(4096))
        .configure(ecomrun::init_routes)
    })
    .bind(format!("{}:{}" ,config_proxy.proxy_host,config_proxy.proxy_port,))?
    .run()
    .await
}

