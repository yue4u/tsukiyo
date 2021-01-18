use crate::events::{
    self,
    model::{Event, EventInput, EventQuery, EventUpdate},
};
use crate::{auth::User, Context};
use juniper::FieldResult;
use juniper::{graphql_object, EmptySubscription};

pub(crate) struct Query;

#[graphql_object(
    context = Context,
)]
impl Query {
    fn api_version() -> &str {
        "0.1"
    }

    fn me(context: &Context) -> FieldResult<Option<User>> {
        Ok(context.user.clone())
    }

    fn event(context: &Context, id: i32) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::service::get(conn, id)?;
        Ok(event)
    }

    fn events(context: &Context, by: Option<EventQuery>) -> FieldResult<Vec<Event>> {
        let conn = context.pool.get()?;
        let events = events::service::list(conn, by)?;
        Ok(events)
    }
}

pub(crate) struct Mutation;

#[graphql_object(
    context = Context,
)]
impl Mutation {
    fn create_event(context: &Context, event: EventInput) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::service::create(conn, event)?;
        Ok(event)
    }

    fn update_event(context: &Context, id: i32, update: EventUpdate) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::service::update(conn, id, update)?;
        Ok(event)
    }

    fn delete_event(context: &Context, id: i32) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::service::delete(conn, id).expect("failed to delete event");
        Ok(event)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
