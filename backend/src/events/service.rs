use super::model::*;
use crate::db::DBConn;
use crate::schema::events;
use diesel::prelude::*;
use std::default::Default;

pub fn create(conn: DBConn, title: &str, body: &str) -> String {
    let new_post = NewEvent {
        title: title.to_string(),
        body: body.to_string(),
        ..Default::default()
    };

    diesel::insert_into(events::table)
        .values(&new_post)
        .get_result::<Event>(&conn)
        .expect("Error saving new post");
    "ok".to_string()
}

pub fn list(conn: DBConn) -> String {
    use crate::schema::events::dsl::*;

    let results = events
        .filter(published.eq(true))
        .limit(5)
        .load::<Event>(&conn)
        .expect("Error loading events");
    serde_json::to_string(&results).unwrap()
}
