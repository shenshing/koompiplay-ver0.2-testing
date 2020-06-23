// extern crate game_back_end;
extern crate diesel;
use crate::models::{Score, PlayerQue};
use crate::establish_connection;
// use diesel::dsl::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::dsl::*;


use userInfo::token::decode_token;
use userInfo::models::ApiKey;

extern crate rocket_contrib;
use rocket_contrib::json::Json;

pub fn hello() {
    println!("Hello from score.rs file");
}


pub fn last_five_game_score(player_email: String) -> Result<Vec<PlayerQue>, String> {
    use crate::schema::users::dsl::{users};
    use crate::schema::users::columns::user_email;
    let mut results: Vec<PlayerQue> = Vec::new();

    let email_pattern = format!("%{}%", format_args!("{}", player_email.clone()));

    let result = users.filter(user_email.like(email_pattern))
        .execute(&establish_connection());

    match result {
        Ok(ok) => {
            let statement = format!("Select * From players Where email='{}' Order By score Desc limit 5;", player_email);
            // println!("ok: {}", ok);
            results = sql_query(statement)
                .get_results(&establish_connection())
                .unwrap();
            // println!("{:#?}", result);
            return Ok(results);
        },
        Err(err) => {
            // println!("{:#?}", err);
            return Err(format!("No user found"));
        }
    }
    // return results;
}

pub fn select_top_ten() -> Result<Vec<PlayerQue>, String> {
    let statement = format!("Select * From players Order By score Desc limit 10;");

    let result = sql_query(statement)
        .get_results(&establish_connection());
        // .unwrap();

    match result {
        Ok(data) => {
            return Ok(data);
        },
        Err(err) => {
            return Err(err.to_string());
        }
    }
    // return result;
}   

#[get("/user-score")]
// pub fn private_score(key: ApiKey) -> Result<Json<PlayerQue>, String> {
pub fn private_score(key: ApiKey) -> Json<Vec<PlayerQue>> {
    let token = key.into_inner();

    let decode = decode_token(token.clone().to_string());
    let email = decode.claims.user_email;

    return Json(last_five_game_score(email).unwrap());
}

#[get("/public-rank")]
pub fn public_rank(key: ApiKey) -> Json<Vec<PlayerQue>> {
    return Json(select_top_ten().unwrap());
}