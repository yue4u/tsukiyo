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

pub fn list(conn: Conn, by: Option<EventQuery>) -> QueryResult<Vec<Event>> {
    let mut query = events::table.into_boxed();
    if let Some(by_event) = by {
        if let Some(_published) = by_event.published {
            query = query.filter(events::published.eq(_published));
        }
        if let Some(_genre) = by_event.genre {
            query = query.filter(events::genre.eq(_genre));
        }
        if let Some(_tag) = by_event.tag {
            query = query.filter(events::genre.eq(_tag));
        }
        if let Some(_limit) = by_event.limit {
            query = query.limit(_limit as i64);
        }
    }
    query.load(&conn)
}
