use super::model::*;
use crate::sql::{
    db::Conn,
    schema::events::{self, dsl::*},
};
use diesel::prelude::*;

pub fn get(conn: Conn, event_id: i32) -> Result<Event, String> {
    Ok(events
        .filter(id.eq(event_id))
        .get_result(&conn)
        .expect("failed to get query create"))
}

pub fn create(conn: Conn, event: EventInput) -> Result<Event, String> {
    if event.title.is_empty() {
        return Err("event title should not be empty".to_owned());
    }
    if event.body.is_empty() {
        return Err("event body should not be empty".to_owned());
    }

    Ok(diesel::insert_into(events::table)
        .values(&event)
        .get_result::<Event>(&conn)
        .expect("failed to execute query create"))
}

pub fn update(conn: Conn, event_id: i32, event: EventUpdate) -> Result<Event, String> {
    Ok(diesel::update(events.filter(id.eq(event_id)))
        .set(&event)
        .get_result(&conn)
        .expect("failed to execute query update"))
}

pub fn delete(conn: Conn, event_id: i32) -> Result<Event, String> {
    Ok(diesel::delete(events.filter(id.eq(event_id)))
        .get_result(&conn)
        .expect("failed to execute query delete"))
}

pub fn list(conn: Conn, by: Option<EventQuery>) -> Result<Vec<Event>, String> {
    let mut query = events::table.into_boxed();
    if let Some(by_event) = by {
        // TODO: simplify this
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
    Ok(query
        .load::<Event>(&conn)
        .expect("failed to execute query delete"))
}
