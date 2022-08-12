use proto::api::{ServerCoder, Ack};
use warp::{Filter, http::StatusCode, Rejection, Reply};
use proto::api;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::convert::Infallible;

const EXTENSION_KEY_CODE: &str = "code";
const CODE_NOT_FOUND: &str = "NOT_FOUND";
const CODE_INTERNAL: &str = "INTERNAL";

pub async fn handle_error(rejection: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let status;
    let err;

    if rejection.is_not_found() {
        status = StatusCode::NOT_FOUND;
        err = crate::Error::NotFound("Route not found.".to_string());
    } else if let Some(_) = rejection.find::<warp::filters::body::BodyDeserializeError>() {
        status = StatusCode::BAD_REQUEST;
        err = crate::Error::InvalidArgument("Invalid Body.".to_string());
    } else if let Some(_) = rejection.find::<warp::reject::MethodNotAllowed>() {
        status = StatusCode::METHOD_NOT_ALLOWED;
        err = crate::Error::InvalidArgument("Invalid HTTP Method.".to_string());
    } else if let Some(e) = rejection.find::<crate::Error>() {
        status = match e {
            crate::Error::InvalidArgument(_) => StatusCode::BAD_REQUEST, // 400
            // Error::AuthenticationRequired => StatusCode::UNAUTHORIZED, // 401
            // Error::PermissionDenied(_) => StatusCode::FORBIDDEN,       // 403
            crate::Error::NotFound(_) => StatusCode::NOT_FOUND, // 404
            // Error::AlreadyExists(_) => StatusCode::CONFLICT,           // 409
            crate::Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
        };
        err = e.to_owned();
    } else {
        status = StatusCode::INTERNAL_SERVER_ERROR;
        err = crate::Error::Internal("".to_string());
    }

  let res = api::Response::<()>::Error(err.to_string());
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, status))
}

pub fn json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Debug)]
pub struct AppState {
    pub service: crate::Service,
}

impl AppState {
    pub fn new(service: crate::Service) -> AppState {
        AppState { service }
    }
}

pub fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn routes(state: Arc<AppState>) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
  let api = warp::path("pr2");
  let api_with_state = api.and(with_state(state));
  let index = api.and(warp::path::end()).and(warp::get()).and_then(index);

  let routes = index.with(warp::log("http_server"))
    .recover(handle_error);

  routes
}

pub async fn index() -> Result<impl warp::Reply, Rejection> {
    let mut data = std::collections::HashMap::new();
    data.insert("hello", "world");

    let res = api::Response::ok(data);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
