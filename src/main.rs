use clap::{crate_version, value_t, App, Arg};
use std::env;

fn main() {
    const DEFAULT_PORT: u32 = 3030;
    const ABOUT: &str = "Simple data collection server for LiftRight";

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

    webserver::run(port).unwrap()
}

mod webserver {
    use super::filters;

    use std::net::SocketAddrV4;
    use warp::Filter;

    use liftright_data_server::LiftrightError;

    #[tokio::main]
    pub async fn run(port: u32) -> Result<(), LiftrightError> {
        let db = liftright_data_server::establish_db_connection().await?;

        let api = filters::rest_api(db).with(warp::log("liftright_data_server"));

        let addr: SocketAddrV4 = format!("0.0.0.0:{}", port)
            .parse()
            .expect("Could not create IP.");

        Ok(warp::serve(api).run(addr).await)
    }
}

mod filters {
    use super::handlers;
    use uuid::Uuid;
    use warp::Filter;

    use liftright_data_server::{
        imurecords::ImuRecordSet, repetition::JsonApiRepetition, survey::Survey,
    };

    pub fn rest_api(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        repetitions_add(db.clone())
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

    fn rtfb_status(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "rtfb_status" / Uuid)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::rtfb_status)
    }

    fn repetitions_add(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "add_repetition")
            .and(warp::put())
            .and(with_db(db))
            .and(json_deserialize::<JsonApiRepetition>())
            .and_then(handlers::add_repetition)
    }

    fn add_imu_records(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "add_imu_records")
            .and(warp::put())
            .and(with_db(db))
            .and(json_deserialize::<ImuRecordSet>())
            .and_then(handlers::add_imu_records)
    }

    fn survey_submit(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "submit_survey")
            .and(warp::post())
            .and(with_db(db))
            .and(json_deserialize::<Survey>())
            .and_then(handlers::submit_survey)
    }

    fn json_deserialize<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: serde::de::DeserializeOwned + Send,
    {
        warp::body::content_length_limit(1024 * 1024).and(warp::body::json())
    }

    fn with_db(
        collection: mongodb::Collection,
    ) -> impl Filter<Extract = (mongodb::Collection,), Error = warp::reject::Rejection> + Clone
    {
        warp::any().map(move || collection.clone()).and_then(
            |collection: mongodb::Collection| async move {
                if true {
                    Ok(collection)
                } else {
                    Err(warp::reject())
                }
            },
        )
    }
}

mod handlers {
    use uuid::Uuid;
    use warp::http;

    use liftright_data_server::{
        imurecords::ImuRecordSet,
        repetition::JsonApiRepetition,
        survey::Survey,
        user::{ExtractUser, User},
        LiftrightError,
    };

    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    struct RtfbJsonReply {
        rtfb_status: bool,
    }

    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    struct RecordsUpdatedJsonReply {
        updated_count: i64,
    }

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

    pub async fn rtfb_status(
        device_id: Uuid,
        collection: mongodb::Collection,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let user = User::new(device_id);
        match user.check_rtfb_status(collection).await {
            Ok(rtfb_status) => Ok(warp::reply::json(&RtfbJsonReply { rtfb_status })),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }

    pub async fn add_repetition(
        collection: mongodb::Collection,
        rep: JsonApiRepetition,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let user = rep.extract_user();

        match user.add_repetition(collection, rep).await {
            Ok(updated_count) => Ok(warp::reply::json(&RecordsUpdatedJsonReply {
                updated_count,
            })),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }

    pub async fn add_imu_records(
        collection: mongodb::Collection,
        imurecords: ImuRecordSet,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let user = imurecords.extract_user();

        match user.add_imu_records(collection, imurecords).await {
            Ok(updated_count) => Ok(warp::reply::json(&RecordsUpdatedJsonReply {
                updated_count,
            })),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }

    pub async fn submit_survey(
        _collection: mongodb::Collection,
        _survey_data: Survey,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if true {
            Err(warp::reject::custom(LiftrightError::UnimplementedError))
        } else {
            Ok(warp::reply())
        }
    }
}
