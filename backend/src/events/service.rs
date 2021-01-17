use super::model::*;
use crate::db::Conn;
use crate::schema::events::{self, dsl::*};
use diesel::prelude::*;

pub fn get(conn: Conn, event_id: i32) -> QueryResult<Event> {
    events.filter(id.eq(event_id)).get_result(&conn)
}

pub fn create(conn: Conn, event: EventInput) -> QueryResult<Event> {
    diesel::insert_into(events::table)
        .values(&event)
        .get_result::<Event>(&conn)
}

pub fn update(conn: Conn, event_id: i32, event: EventUpdate) -> QueryResult<Event> {
    diesel::update(events.filter(id.eq(event_id)))
        .set(&event)
        .get_result(&conn)
}

pub fn delete(conn: Conn, event_id: i32) -> QueryResult<Event> {
    diesel::delete(events.filter(id.eq(event_id))).get_result(&conn)
}

pub fn list(conn: Conn) -> QueryResult<Vec<Event>> {
    events
        .filter(published.eq(true))
        .limit(5)
        .load::<Event>(&conn)
}
