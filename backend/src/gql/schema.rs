use crate::{
    auth::User,
    contacts::{self, Contact, ContactInput, ContactQuery, ContactUpdate},
    events::{self, Event, EventInput, EventQuery, EventUpdate},
    Context,
};
use juniper::{graphql_object, EmptySubscription, FieldResult};

impl juniper::Context for Context {}

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

    fn contact(context: &Context, id: i32) -> FieldResult<Contact> {
        let conn = context.pool.get()?;
        let contact = contacts::service::get(conn, id)?;
        Ok(contact)
    }

    fn contacts(context: &Context, by: Option<ContactQuery>) -> FieldResult<Vec<Contact>> {
        let conn = context.pool.get()?;
        let contacts = contacts::service::list(conn, by)?;
        Ok(contacts)
    }
}

pub(crate) struct Mutation;

#[graphql_object(
    context = Context,
)]
impl Mutation {
    fn create_event(context: &Context, event: EventInput) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::create(conn, event)?;
        Ok(event)
    }

    fn update_event(context: &Context, id: i32, update: EventUpdate) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::update(conn, id, update)?;
        Ok(event)
    }

    fn delete_event(context: &Context, id: i32) -> FieldResult<Event> {
        let conn = context.pool.get()?;
        let event = events::delete(conn, id)?;
        Ok(event)
    }

    fn create_contact(context: &Context, contact: ContactInput) -> FieldResult<Contact> {
        let conn = context.pool.get()?;
        let contact = contacts::create(conn, contact)?;
        Ok(contact)
    }

    fn update_contact(context: &Context, id: i32, update: ContactUpdate) -> FieldResult<Contact> {
        let conn = context.pool.get()?;
        let contact = contacts::update(conn, id, update)?;
        Ok(contact)
    }

    fn delete_contact(context: &Context, id: i32) -> FieldResult<Contact> {
        let conn = context.pool.get()?;
        let contact = contacts::delete(conn, id)?;
        Ok(contact)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
