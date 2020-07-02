#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// use userInfo::token::decode_token;
// use userInfo::token::Claims;
// use jsonwebtoken::Validation;

// use userInfo::static_rocket_route_info_for_get_profile;
// use userInfo::static_rocket_route_info_for_upload;
// use userInfo::get_user_by_name_password;
use userInfo::establish_connection;
use userInfo::{insert_all_type_of_user};
use userInfo::models::Test_Users;


use userInfo::{Find, filter_user};

use std::fs::File;
use std::io::prelude::*;

extern crate rocket_cors;
// use userInfo::models::sensitive
use userInfo::models::static_rocket_route_info_for_sensitive;
use userInfo::static_rocket_route_info_for_all_type_register;
use userInfo::static_rocket_route_info_for_all_type_login;
use userInfo::static_rocket_route_info_for_return_st;
use userInfo::models::is_valid_token;
use diesel::dsl::sql_query;
use diesel::query_dsl::RunQueryDsl;
use jsonwebtoken::Validation;

use userInfo::models::stringObj;
use userInfo::token::decode_token;

// #[macro_use]
// extern crate json;

use jsonwebtoken::encode;


fn main() {
    // let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrb29tcGlQbGF5IiwiZXhwIjoxNjAwNTk0MjQwLCJpYXQiOjE1ODY0MjQ2NDAsImlzcyI6Imtvb21waVBsYXkiLCJzdWIiOiJsb2dpbiIsInVzZXJfbmFtZSI6InNoaW5nIiwidXNlcl9wYXNzd29yZCI6IjEyMyIsInVzZXJfcm9sZSI6IlVzZXIifQ.mi7rDF5AcQcQeUhKJhSTGKTvH5_W9tAE5TydZcPx8jU");
    // let dec_res = jsonwebtoken::decode::<Claims>(&token, "secret".as_ref(), &Validation::default()).unwrap();
    // println!("{:#?}", dec_res);

        // let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();
        
        // rocket::ignite()
        //     // .mount("/", routes![get_profile])
        //     // .mount("/", routes![upload])
        //     // .mount("/", routes![])
        //     // .mount("/", routes![sensitive])
        //     // .mount("/", routes![all_type_register])
        //     // .mount("/", routes![all_type_login])
        //     .mount("/", routes![return_st])
        //     // .attach(cors)
        //     .launch();

    // let file_name = String::from("a.txt");
    // let file_fmt = format!("/home/koompi/Documents/koompi-play-production/userInfo/image-bank/{}", file_name);
    // let mut file = File::create(file_fmt).unwrap();
    // file.write_all(b"Hello World").unwrap();

    // let token = format!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrb29tcGlQbGF5IiwiZXhwIjoxNjAyMTQxMzU1LCJpYXQiOjE1ODc5NzE3NTUsImlzcyI6Imtvb21waVBsYXkiLCJzdWIiOiJsb2dpbiIsInVzZXJfbmFtZSI6InBlcnNvbjMiLCJ1c2VyX3Bhc3N3b3JkIjoiMTIzIiwidXNlcl9yb2xlIjoiVXNlciJ9.DlrZXo_jsZQ6nGKRwMwTG19pQo1emOMmtyeQ4F_X9Nw");
    // let res = filter_user(token);
    
    // match res {
    //     Find::Found => println!("User Found"),
    //     Find::Notfound => println!("User not Found"),
    // }

    // get_user_by_name_password(String::from("shing"), String::from("123"));

//local insert

    // let user = Test_Users {
    //     user_name:          String::from("second_user"),
    //     user_external_id:   Some(String::from("external id")),
    //     user_gender:        String::from("Female"),
    //     user_email:         Some(String::from("second@gmail.com")),
    //     user_password:      Some(String::from("12345678")),
    //     user_profile:       Some(String::from("second.image")),
    //     user_role:          String::from("User"),
    //     phone_number:       Some(String::from("012 543 665")),
    //     login_type:         String::from("local")
    // };

    // let conn = establish_connection();

    // let result = insert_all_type_of_user(&conn, user);

    // println!("result: {:#?}", result);
     
    // let st = String::new();
    // if(st.is_empty()) {
    //     println!("is null");
    // } else {
    //     println!("is not null");
    // }

    // let statement = format!("Select * From test_users Where login_type = 'telegram';");
    // let result = sql_query(statement)
    //     .execute(&conn);

    // println!("result: {:#?}", result);


    // let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrb29tcGlQbGF5IiwiZXhwIjoxNjA2MDE3MTI3LCJ1c2VyX2VtYWlsIjoic29sbzRAZ21haWwuY29tIiwidXNlcl9yb2xlIjoiVXNlciJ9.T4xgQ4SdmzcNUg7M-DemNANeUaB3alO0NR5oIEfc1Zs");
    // let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrb29tcGlQbGF5IiwiZXhwIjoxNTkxODQ4NjIxLCJ1c2VyX2VtYWlsIjoic29sbzRAZ21haWwuY29tIiwidXNlcl9yb2xlIjoiVXNlciJ9.td0dfUaaJR_Mr9QrTYuQcF0GzEg0LEhCOWgHK3Q3gCg");
    // let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrb29tcGlQbGF5IiwiZXhwIjoxNTkxOTM2MjY5LCJ1c2VyX2VtYWlsIjoic29sbzRAZ21haWwuY29tIiwidXNlcl9yb2xlIjoiVXNlciJ9.pYdXahhUa_hZmnIy-pTDdw9SGz6cFs_iDGzRcX2UXJU");
    // let result = is_valid_token(&token);
    // println!("{}", result);


    // let validation = Validation::default();
    // println!("{:#?}", validation);

    // println!("Hello World");


    let st = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJrb29tcGlQbGF5IiwiZXhwIjoxNTkzMDc0NzQxLCJ1c2VyX2VtYWlsIjoic2hpbmcxQGdtYWlsLmNvbSIsInVzZXJfcm9sZSI6IlVzZXIifQ.UUG291rKWiEWjJHVZnhYkQzWnOFCFervJD8w5x6VqCw");
    let result = decode_token(st);
    println!("{:#?}", result);
}