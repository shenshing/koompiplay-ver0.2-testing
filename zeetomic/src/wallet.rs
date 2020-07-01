// #![feature(proc_macro_hygiene, decl_macro)]
// #[macro_use] extern crate rocket;
// #[macro_use]
// extern crate rocket;

use rocket_contrib::json::Json;
use crate::models::{Wallet, stringObj};
use crate::establish_connection;
use diesel::prelude::*;
#[post("/create-wallet", data="<new_wallet>")]
pub fn save_wallet_to_db(new_wallet: Json<Wallet>) -> Json<stringObj> {
    use crate::schema::zee_wallet;

    let wallet_data = new_wallet.into_inner();

    let insert_result = diesel::insert_into(zee_wallet::table)
        .values(&wallet_data)
        .execute(&establish_connection());
    // println!("{:#?}", new_wallet);

    // println!("Hello World");
    match insert_result {
        Ok(ok) => return Json(
            stringObj {
                string: String::from("Successfully create wallet")
            } 
        ),
        Err(err) => return Json(
            stringObj {
                string: String::from("Something went wrong")
            }
        )
    }
}