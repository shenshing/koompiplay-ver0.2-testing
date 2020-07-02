use userInfo::models::ApiKey;
use userInfo::token::decode_token;

use rocket_contrib::json::Json;
use crate::models::{Wallet, stringObj};
use crate::establish_connection;
use diesel::prelude::*;
#[post("/create-wallet", data="<new_wallet>")]
pub fn save_wallet_to_db(key: ApiKey, new_wallet: Json<Wallet>) -> Json<stringObj> {
    use crate::schema::zee_wallet;

    // let wallet_data = new_wallet.into_inner().clone();

    let token = key.into_inner();
    let claim = decode_token(token.clone().to_string());
    let save_email = claim.claims.user_email;
    let wallet_ = new_wallet.into_inner();

    let save_wallet = Wallet {
        wallet_id: wallet_.wallet_id,
        wallet: wallet_.wallet,
        email:  save_email
    };

    let insert_result = diesel::insert_into(zee_wallet::table)
        .values(&save_wallet)
        .execute(&establish_connection());

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