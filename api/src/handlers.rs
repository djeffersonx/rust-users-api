use log::{trace, warn};
use models::user::User;
use tide::{Body, Request, Response, StatusCode};

use crate::error_response::ErrorResponse;
use crate::state::State;
use uuid::Uuid;

pub(crate) async fn get(req: Request<State>) -> tide::Result {
    let id: String = match req.param("id") {
        Ok(id) => id,
        Err(e) => {
            trace!("Bad Request: {:?}", e);
            let mut res = Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    };

    match req.state().db().find_by_id(id).await {
        Ok(user) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&user)?);
            Ok(res)
        }
        Err(e) => {
            warn!("Error getting user from database: {:?}", e);
            let mut res = Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}

pub(crate) async fn get_all(req: Request<State>) -> tide::Result {
    match req.state().db().find_all().await {
        Ok(users) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&users)?);
            Ok(res)
        }
        Err(e) => {
            warn!("Error getting users from database: {:?}", e);
            let mut res = Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}

pub(crate) async fn save(mut req: Request<State>) -> tide::Result<Response> {
    let user: User = match req.body_json().await {
        Ok(user) => user,
        Err(e) => {
            trace!("Bad Request: {:?}", e);
            let mut res = Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    };

    match req.state().db().create(&user).await {
        Ok(()) => Ok(Response::new(StatusCode::Created)),
        Err(e) => {
            warn!("Error creating user from database: {:?}", e);
            let mut res = Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}