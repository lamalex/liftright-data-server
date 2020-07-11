use std::env;
use clap::{crate_version, value_t, App, Arg};

fn main() {
    const DEFAULT_PORT: u32 = 3030;
    const ABOUT: &'static str = "Simple data collection server for LiftRight";

    let opts = App::new("liftright data server")
        .version(crate_version!())
        .author("Alex L. Launi <alaun001@odu.edu>")
        .about(ABOUT)
        .arg(
            Arg::with_name("port")
                .help("Port to listen on (default 3030)")
                .short("p")
                .long("port")
                .takes_value(true)
                .value_name("PORT"),
        )
        .get_matches();
        
    let port = value_t!(opts.value_of("port"), u32).unwrap_or(DEFAULT_PORT);

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=liftright_data_server=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "liftright_data_server=info");
    }
    pretty_env_logger::init();

    webserver::run(port)
}

mod webserver {
    use warp::Filter;
    use super::filters;
    use std::net::SocketAddrV4;
    
    #[tokio::main]
    pub async fn run(port: u32) {
        let db = liftright_data_server::establish_connection();

        let api = filters::repetitions(db);
        let routes = api.with(warp::log("liftright_data_server"));

        let addr: SocketAddrV4 = format!("0.0.0.0:{}", port).parse()
            .expect("Could not create IP.");
        
        warp::serve(routes).run(addr).await
    }
}

mod filters {
    use uuid::Uuid;
    use warp::Filter;
    use super::handlers;

    use liftright_data_server::repetition::{NewRepetition};
    use liftright_data_server::{DbPool, DbPooledConnection};

    pub fn repetitions(db: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        repetitions_create(db.clone())
        .or(rtfb_status(db))
        .or(hello())
    }

    fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "heartbeat")
            .and(warp::get())
            .and_then(handlers::heartbeat)
    }

    fn repetitions_create(db: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "add_repetition")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_repetition)
    }

    fn rtfb_status(db: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "rtfb_status" / Uuid)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::rtfb_status)
    }

    fn json_body() -> impl Filter<Extract = (NewRepetition,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn with_db(pool: DbPool) -> impl Filter<Extract = (DbPooledConnection,), Error = warp::reject::Rejection> + Clone {
        warp::any()
            .map(move || pool.clone())
            .and_then(|pool: DbPool| async move {
                match pool.get() {
                    Ok(conn) => Ok(conn),
                    Err(_) => Err(warp::reject())
                }
            })
    }
}

mod handlers {
    use warp::http;
    use uuid::Uuid;

    use liftright_data_server::user::User;
    use liftright_data_server::DbPooledConnection;
    use liftright_data_server::repetition::{Repetition, NewRepetition};

    pub async fn heartbeat() -> Result<impl warp::Reply, warp::Rejection> {
        let now = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).expect("System time is before epoch!").as_secs();
        Ok(warp::reply::with_status(
            format!("{}", now),
            http::StatusCode::OK
        ))
    }

    pub async fn create_repetition(rep: NewRepetition, conn: DbPooledConnection) -> Result<impl warp::Reply, warp::Rejection> {
        match Repetition::create(&conn, rep) {
            Ok(_) => Ok(warp::reply::with_status(
                "good job",
                http::StatusCode::CREATED,
            )),
            Err(_) => Err(warp::reject())
        }
    }

    pub async fn rtfb_status(uuid: Uuid, conn: DbPooledConnection) -> Result<impl warp::Reply, std::convert::Infallible> {
        match User::check_rtfb_status(&conn, &uuid) {
            Ok(rtfb) => Ok(warp::reply::json(&rtfb)),
            Err(_) => Ok(warp::reply::json(&false))
        }
    }
}
