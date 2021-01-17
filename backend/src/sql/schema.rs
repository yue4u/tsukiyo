table! {
    admins (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

table! {
    contacts (id) {
        id -> Int4,
        title -> Varchar,
        name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        body -> Text,
        created_at -> Nullable<Timestamp>,
        checked -> Bool,
    }
}

table! {
    events (id) {
        id -> Int4,
        slug -> Nullable<Varchar>,
        title -> Varchar,
        body -> Text,
        genre -> Nullable<Varchar>,
        tag -> Nullable<Varchar>,
        fee -> Nullable<Int4>,
        ogp_img -> Nullable<Varchar>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        publish_time -> Nullable<Timestamp>,
        page_view -> Nullable<Int4>,
        creator_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        published -> Bool,
        memo -> Nullable<Varchar>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

joinable!(events -> admins (creator_id));

allow_tables_to_appear_in_same_query!(admins, contacts, events, posts,);
