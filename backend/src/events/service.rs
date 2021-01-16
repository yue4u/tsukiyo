use super::model::*;
use crate::schema::events;
use diesel::prelude::*;
use std::default::Default;

pub fn create(title: &str, body: &str) -> String {
    let conn = crate::db::get_connection();
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

pub fn list() -> String {
    use crate::schema::events::dsl::*;
    let conn = crate::db::get_connection();

    let results = events
        .filter(published.eq(true))
        .limit(5)
        .load::<Event>(&conn)
        .expect("Error loading events");

    serde_json::to_string(&results).unwrap()
}
