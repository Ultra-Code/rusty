mod custom_response;
mod errors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result, error, get, guard, http,
    middleware::Logger, post, web,
};
use serde::Deserialize;
use std::cell::Cell;
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicUsize, Ordering},
};

// state is accessible as a read-only reference. If you need mutable
// access to state, you must use interior mutable pattern .ie Cell, RefCell
// And to share state across App instances .ie (threads) use concurrency
// primatives .ie Atomics, Mutex, RwLock
struct AppState {
    app_name: String,
    // <- Mutex is necessary to mutate safely across threads
    counter: RwLock<u32>,
}

#[derive(Clone)]
struct AtomicAppState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

#[get("/show_count")]
#[allow(clippy::future_not_send)]
async fn show_count(data: web::Data<AtomicAppState>) -> impl Responder {
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/add_one")]
#[allow(clippy::future_not_send)]
async fn add_one(data: web::Data<AtomicAppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);

    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    // <- get counter's MutexGuard
    let mut instance_count = data.counter.write().unwrap();
    // <- access counter inside MutexGuard
    *instance_count += 1;
    HttpResponse::Ok().body(format!(
        "Hello {app_name}!\nWith {instance_count} instances"
    ))
}

#[get("/count")]
async fn count(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "The count of is {:?} instances",
        &data.counter.read()
    ))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// this function could be located in a different module
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config_submit_json(cfg: &mut web::ServiceConfig) {
    // limits the size of the payload to 4kb and uses a custom error handler
    let json_config =
        web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(
                    err,
                    HttpResponse::Conflict().finish(),
                )
                .into()
            });

    cfg.app_data(json_config).service(submit);
}

#[derive(Deserialize)]
struct UserInfo {
    user_id: u32,
    username: String,
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{username}")] // <- define path parameters
async fn path_extractor(path: web::Path<UserInfo>) -> Result<String> {
    // if path uses structure web::Path<(u32, String)>
    // let (user_id, friend) = path.into_inner();

    Ok(format!(
        "Welcome {}, user_id {}!",
        path.username, path.user_id
    ))
}

/// Extract typed information from a request's body
/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<UserInfo>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// The Query<T> type provides extraction functionality for the request's query parameters.
// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/query_params")]
async fn query_params(info: web::Query<UserInfo>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use log::debug;
    unsafe {
        std::env::set_var("RUST_LOG", "info,backy=debug,backy::errors=error");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();
    debug!("In main, starting backy server");
    // To share data across new App instances constructed by HttpServer
    // for each thread the data must exist outside of HttpServer
    // else a new copy of the data would be created for each App instance
    let data = web::Data::new(AppState {
        app_name: String::from("Backy Web"),
        counter: RwLock::new(0),
    });

    let atomic_data = AtomicAppState {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger) // register logger middleware
            .configure(config)
            .configure(config_submit_json)
            .service(index)
            .app_data(data.clone())
            .service(path_extractor)
            .service(
                web::scope("/atomic")
                    .app_data(web::Data::new(atomic_data.clone()))
                    .service(add_one)
                    .service(show_count),
            )
            .service(
                web::scope("/guard")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route(
                        "",
                        web::to(|| async { HttpResponse::Ok().body("www") }),
                    ),
            )
            .service(
                web::scope("/guard")
                    .guard(guard::Host("users.rust-lang.com"))
                    .route(
                        "",
                        web::to(|| async { HttpResponse::Ok().body("user") }),
                    ),
            )
            .service(web::scope("/api").configure(scoped_config))
            .service(echo)
            .service(count)
            .service(query_params)
            .service(errors::index)
            .service(errors::helper)
            .route("/hey", web::get().to(manual_hello))
    })
    .keep_alive(http::KeepAlive::Os)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
