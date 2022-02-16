use rocket::*;
use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::io::{
    Error,
    ErrorKind::{Interrupted, InvalidData, InvalidInput, NotFound},
};

use crate::db::{models::info::Info, Database};
#[get("/info")]
pub fn get_info() -> Result<Json<Vec<Info>>, Json<Error>> {
    match Database::default()
        .unwrap()
        .connect()
        .unwrap()
        .select_table::<Info>("info".to_owned())
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Json(Error::new(Interrupted, format!("{}", err)))),
    }
}
