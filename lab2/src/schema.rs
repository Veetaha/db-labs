table! {
    news (id) {
        id -> Int4,
        creator_id -> Int4,
        body -> Text,
        promo_img_id -> Nullable<Text>,
        last_update_date -> Timestamp,
        creation_date -> Timestamp,
        title -> Text,
    }
}

table! {
    news_comments (id) {
        id -> Int4,
        commentator_id -> Int4,
        news_id -> Int4,
        body -> Text,
        creation_date -> Timestamp,
        last_update_date -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::entities::RatingValueMapping;

    news_ratings (id) {
        id -> Int4,
        rater_id -> Int4,
        news_id -> Int4,
        value -> RatingValueMapping,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::entities::RoleMapping;

    users (id) {
        id -> Int4,
        password_hash -> Text,
        avatar_img_id -> Nullable<Text>,
        login -> Text,
        name -> Text,
        role -> RoleMapping,
        last_update_date -> Timestamp,
        creation_date -> Timestamp,
    }
}

joinable!(news -> users (creator_id));
joinable!(news_comments -> news (news_id));
joinable!(news_comments -> users (commentator_id));
joinable!(news_ratings -> news (news_id));
joinable!(news_ratings -> users (rater_id));

allow_tables_to_appear_in_same_query!(
    news,
    news_comments,
    news_ratings,
    users,
);
