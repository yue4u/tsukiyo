use super::model::*;
use crate::db::DBConn;
use crate::schema::events::{self, dsl::*};
use diesel::prelude::*;

pub fn create(conn: DBConn, event: EventInput) -> String {
    diesel::insert_into(events::table)
        .values(&event)
        .get_result::<Event>(&conn)
        .expect("Error saving new post");
    "ok".to_string()
}

pub fn update(conn: DBConn, event: EventUpdate) -> String {
    diesel::delete(events.filter(id.eq(event.id)))
        .execute(&conn)
        .expect("Error delete new post");
    "ok".to_string()
}

pub fn delete(conn: DBConn, event_id: i32) -> String {
    diesel::delete(events.filter(id.eq(event_id)))
        .execute(&conn)
        .expect("Error delete new post");
    "ok".to_string()
}

pub fn list(conn: DBConn) -> String {
    let results = events
        .filter(published.eq(true))
        .limit(5)
        .load::<Event>(&conn)
        .expect("Error loading events");
    serde_json::to_string(&results).unwrap()
}
