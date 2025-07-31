use airdrop::middleware::auth;
use airdrop::utils::logger::init_logger;
use clap::Parser;
use actix_web::{ 
    web, App, HttpServer
};

use airdrop::{config::config::AppConfig};
use airdrop::db::{mysql, redis};
use airdrop::app::state::AppState;
use airdrop::routes;
use tracing_actix_web::TracingLogger;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short,long,default_value_t= String::from("config/local.toml"), help = "Path to the configuration file")]
    config: String,
}

#[actix_web::main]

async fn main() -> std::io::Result<()>  {
    let args = Args::parse();
    println!("Load Config from file: {:?}", args.config);
    
    let app_config = AppConfig::new(&args.config);
    println!("App Config: {:?}", app_config.to_json());

    init_logger(app_config.log());
    tracing::info!("Logger initialized with config: {:?}", app_config.log());

    let mysql_pool = mysql::init_mysql_pool(app_config.mysql()).await;
    let redis_pool = redis::init_redis_pool(app_config.redis());
    let app_state = web::Data::new(AppState {
        mysql: mysql_pool,
        redis: redis_pool.clone(),
        config: app_config.clone(),
    });
    let server_config = app_config.server().clone();

    tracing::info!("Starting server at {}:{}", server_config.host, server_config.port);


    let jwt_factory = auth::JwtMiddleware{
        secret:app_config.jwt().secret.clone(),
        redis_pool:redis_pool.clone()
    };

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(|cfg| routes::init(cfg,jwt_factory.clone()))
            .wrap(TracingLogger::default())
    })
    .bind((server_config.host, server_config.port))?
    .run()
    .await
   

}
