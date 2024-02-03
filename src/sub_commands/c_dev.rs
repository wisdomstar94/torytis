use axum::Router;

use crate::run_command;
mod routes;

#[derive(clap::Args)]
#[command(
    about="torytis 를 개발용으로 로컬에 구동합니다.", 
    long_about = None
)]
pub struct CliArgs {
    #[arg(short='p', long="port")]
    port: Option<u32>,
}

pub async fn run(args: CliArgs) {
    run_command("npm run build -- --flat=false").unwrap();

    let port = if let Some(p) = args.port {
        p
    } else {
        3000
    };

    let app = Router::new()
        .nest("/", routes::root::routes())
        .nest("/category", routes::category::routes())
        .nest("/tag", routes::tag::routes())
        .nest("/notice", routes::notice::routes())
        .nest("/virtualcdn", routes::virtualcdn::routes())
        // .nest("/test", test::routes())
        // .route_layer(middleware::from_fn(middlewares::header_auth_check::middleware))
        ;

    let url = format!("http://localhost:{}", port);
    println!("");
    println!("torytis dev server start..!");
    println!("");
    println!("#########################################");
    println!("");
    println!("          {}          ", url);
    println!("");
    println!("#########################################");
    println!("");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port).as_str()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

