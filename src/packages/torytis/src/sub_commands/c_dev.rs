use axum::Router;

use crate::sub_commands::c_build::build;
// use commander::functions::run_command::run_command;
// use crate::common::get_script_watch_path_buf;
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
    build(&false);

    let port = if let Some(p) = args.port {
        p
    } else {
        3000
    };

    let app = Router::new()
        .merge(routes::category::routes())
        .merge(routes::tag::routes())
        .merge(routes::notice::routes())
        .merge(routes::search::routes())
        .merge(routes::guestbook::routes()) 
        .merge(routes::virtualcdn::routes())
        .merge(routes::tistorycdn::routes())
        .merge(routes::processer::routes())
        .merge(routes::root::routes())
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

    axum::serve(listener, app).await.unwrap()
    // let a = tokio::task::spawn(async move {
    //     let port = if let Some(p) = args.port {
    //         p
    //     } else {
    //         3000
    //     };
    
    //     let app = Router::new()
    //         .nest("/category", routes::category::routes())
    //         .nest("/tag", routes::tag::routes())
    //         .nest("/notice", routes::notice::routes())
    //         .nest("/search", routes::search::routes())
    //         .nest("/guestbook", routes::guestbook::routes()) 
    //         .nest("/virtualcdn", routes::virtualcdn::routes())
    //         .nest("/tistorycdn", routes::tistorycdn::routes())
    //         .nest("/", routes::root::routes())
    //         // .nest("/test", test::routes())
    //         // .route_layer(middleware::from_fn(middlewares::header_auth_check::middleware))
    //         ;
    
    //     let url = format!("http://localhost:{}", port);
    //     println!("");
    //     println!("torytis dev server start..!");
    //     println!("");
    //     println!("#########################################");
    //     println!("");
    //     println!("          {}          ", url);
    //     println!("");
    //     println!("#########################################");
    //     println!("");
    
    //     let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port).as_str()).await.unwrap();
    
    //     axum::serve(listener, app).await
    // });

    // let b = tokio::task::spawn(async {
    //     let watch_script_js_path = get_script_watch_path_buf();
    //     let command = format!("node {}", watch_script_js_path.to_str().unwrap());
    //     println!("> {}", command);
    //     let _ = run_command(command.as_str()).unwrap();
    // });

    // let (res_a, res_b) = tokio::join!(a, b);

    // res_a.unwrap().unwrap();
    // res_b.unwrap();
}
