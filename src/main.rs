use clap::{crate_version, value_t, App, Arg};
use std::env;

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
    use super::filters;
    use std::net::SocketAddrV4;
    use warp::Filter;

    #[tokio::main]
    pub async fn run(port: u32) {
        let db = liftright_data_server::establish_connection();

        let api = filters::rest_api(db).with(warp::log("liftright_data_server"));

        let addr: SocketAddrV4 = format!("0.0.0.0:{}", port)
            .parse()
            .expect("Could not create IP.");

        warp::serve(api).run(addr).await
    }
}

mod filters {
    use super::handlers;
    use uuid::Uuid;
    use warp::Filter;

    use liftright_data_server::imurecords::ImuRecordSet;
    use liftright_data_server::repetition::NewRepetition;
    use liftright_data_server::survey::Survey;
    use liftright_data_server::{DbPool, DbPooledConnection};

    pub fn rest_api(
        db: DbPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        repetitions_create(db.clone())
            .or(rtfb_status(db.clone()))
            .or(survey_submit(db.clone()))
            .or(add_imu_records(db))
            .or(heartbeat())
    }

    fn heartbeat() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "heartbeat")
            .and(warp::get())
            .and_then(handlers::heartbeat)
    }

    fn repetitions_create(
        db: DbPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "add_repetition")
            .and(warp::put())
            .and(json_deserialize::<NewRepetition>())
            .and(with_db(db))
            .and_then(handlers::create_repetition)
    }

    fn rtfb_status(
        db: DbPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "rtfb_status" / Uuid)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::rtfb_status)
    }

    fn survey_submit(
        db: DbPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "submit_survey")
            .and(warp::post())
            .and(json_deserialize::<Survey>())
            .and(with_db(db))
            .and_then(handlers::submit_survey)
    }

    fn add_imu_records(
        db: DbPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "add_imu_records")
            .and(warp::put())
            .and(json_deserialize::<ImuRecordSet>())
            .and(with_db(db))
            .and_then(handlers::add_imu_records)
    }

    fn json_deserialize<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: serde::de::DeserializeOwned + Send,
    {
        warp::body::content_length_limit(1024 * 1024).and(warp::body::json())
    }

    fn with_db(
        pool: DbPool,
    ) -> impl Filter<Extract = (DbPooledConnection,), Error = warp::reject::Rejection> + Clone {
        warp::any()
            .map(move || pool.clone())
            .and_then(|pool: DbPool| async move {
                match pool.get() {
                    Ok(conn) => Ok(conn),
                    Err(_) => Err(warp::reject()),
                }
            })
    }
}

mod handlers {
    use uuid::Uuid;
    use warp::http;

    use liftright_data_server::repetition::{NewRepetition, Repetition};
    use liftright_data_server::user::User;
    use liftright_data_server::DbPooledConnection;
    use liftright_data_server::{imurecords, imurecords::ImuRecordSet};
    use liftright_data_server::{survey, survey::Survey};

    pub async fn heartbeat() -> Result<impl warp::Reply, warp::Rejection> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("System time is before epoch!")
            .as_secs();
        Ok(warp::reply::with_status(
            format!("{}", now),
            http::StatusCode::OK,
        ))
    }

    pub async fn create_repetition(
        rep: NewRepetition,
        conn: DbPooledConnection,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match Repetition::create(&conn, rep) {
            Ok(_) => Ok(warp::reply::with_status(
                "good job",
                http::StatusCode::CREATED,
            )),
            Err(_) => Err(warp::reject()),
        }
    }

    pub async fn rtfb_status(
        uuid: Uuid,
        conn: DbPooledConnection,
    ) -> Result<impl warp::Reply, std::convert::Infallible> {
        match User::check_rtfb_status(&conn, &uuid) {
            Ok(rtfb) => Ok(warp::reply::json(&rtfb)),
            Err(_) => Ok(warp::reply::json(&false)),
        }
    }

    pub async fn submit_survey(
        survey_data: Survey,
        conn: DbPooledConnection,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match survey::submit(&conn, survey_data) {
            Ok(_) => Ok(warp::reply::with_status(
                "thanks",
                http::StatusCode::CREATED,
            )),
            Err(_) => Err(warp::reject()),
        }
    }

    pub async fn add_imu_records(
        imurecords: ImuRecordSet,
        conn: DbPooledConnection,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match imurecords::add(&conn, imurecords) {
            Ok(_) => Ok(warp::reply()),
            Err(_) => Err(warp::reject()),
        }
    }
}
