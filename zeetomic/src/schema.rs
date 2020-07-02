table! {
    player (id) {
        id -> Int4,
        playername -> Varchar,
        score -> Int4,
        playdate -> Timestamp,
        email -> Varchar,
        quiz_category -> Varchar,
    }
}

table! {
    players (id) {
        id -> Int4,
        playername -> Varchar,
        score -> Int4,
        playdate -> Timestamp,
        email -> Varchar,
    }
}

table! {
    questions (question_id) {
        question_id -> Int4,
        question -> Varchar,
        correct_answer -> Varchar,
        incorrect_answer1 -> Varchar,
        incorrect_answer2 -> Varchar,
        incorrect_answer3 -> Varchar,
        incorrect_answer4 -> Varchar,
        incorrect_answer5 -> Varchar,
        category -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_external_id -> Nullable<Varchar>,
        user_name -> Varchar,
        user_gender -> Varchar,
        user_email -> Nullable<Varchar>,
        user_password -> Nullable<Varchar>,
        create_date -> Timestamp,
        user_profile -> Nullable<Varchar>,
        user_role -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        login_type -> Varchar,
    }
}

table! {
    zee_wallet (id) {
        id -> Int4,
        wallet_id -> Varchar,
        wallet -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    player,
    players,
    questions,
    users,
    zee_wallet,
);
