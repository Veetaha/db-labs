table! {
    news (id) {
        id -> Int4,
        creator_id -> Int4,
        body -> Text,
        promo_img_id -> Nullable<Text>,
        last_update_date -> Timestamp,
        creation_date -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        password_hash -> Text,
        avatar_img_id -> Nullable<Text>,
        login -> Text,
        name -> Text,
        role -> Role,
        last_update_date -> Timestamp,
        creation_date -> Timestamp,
    }
}

joinable!(news -> users (creator_id));

allow_tables_to_appear_in_same_query!(
    news,
    users,
);
