import gql from "graphql-tag";
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /** NaiveDateTime */
  NaiveDateTime: number;
};

/** Event data including admin info */
export type Event = {
  id: Scalars["Int"];
  slug?: Maybe<Scalars["String"]>;
  title: Scalars["String"];
  body: Scalars["String"];
  genre?: Maybe<Scalars["String"]>;
  tag?: Maybe<Scalars["String"]>;
  fee?: Maybe<Scalars["Int"]>;
  ogpImg?: Maybe<Scalars["String"]>;
  startAt?: Maybe<Scalars["NaiveDateTime"]>;
  endAt?: Maybe<Scalars["NaiveDateTime"]>;
  publishAt?: Maybe<Scalars["NaiveDateTime"]>;
  updatedAt?: Maybe<Scalars["NaiveDateTime"]>;
  pageView: Scalars["Int"];
  creatorId?: Maybe<Scalars["Int"]>;
  createdAt: Scalars["NaiveDateTime"];
  published: Scalars["Boolean"];
  memo?: Maybe<Scalars["String"]>;
};

export type EventUpdate = {
  slug?: Maybe<Scalars["String"]>;
  title?: Maybe<Scalars["String"]>;
  body?: Maybe<Scalars["String"]>;
  genre?: Maybe<Scalars["String"]>;
  tag?: Maybe<Scalars["String"]>;
  fee?: Maybe<Scalars["Int"]>;
  ogpImg?: Maybe<Scalars["String"]>;
  startAt?: Maybe<Scalars["NaiveDateTime"]>;
  endAt?: Maybe<Scalars["NaiveDateTime"]>;
  publishAt?: Maybe<Scalars["NaiveDateTime"]>;
  updatedAt?: Maybe<Scalars["NaiveDateTime"]>;
  pageView?: Maybe<Scalars["Int"]>;
  creatorId?: Maybe<Scalars["Int"]>;
  published?: Maybe<Scalars["Boolean"]>;
  memo?: Maybe<Scalars["String"]>;
};

export type ContactUpdate = {
  checked: Scalars["Boolean"];
};

export type ContactQuery = {
  searchString?: Maybe<Scalars["String"]>;
  checked?: Maybe<Scalars["Boolean"]>;
};

export type EventQuery = {
  genre?: Maybe<Scalars["String"]>;
  tag?: Maybe<Scalars["String"]>;
  published?: Maybe<Scalars["Boolean"]>;
  limit?: Maybe<Scalars["Int"]>;
};

/** A new contact */
export type Contact = {
  id: Scalars["Int"];
  title: Scalars["String"];
  name: Scalars["String"];
  email: Scalars["String"];
  phone?: Maybe<Scalars["String"]>;
  body: Scalars["String"];
  createdAt: Scalars["NaiveDateTime"];
  checked: Scalars["Boolean"];
};

/** Event input for creating event */
export type EventInput = {
  slug?: Maybe<Scalars["String"]>;
  title: Scalars["String"];
  body: Scalars["String"];
  genre?: Maybe<Scalars["String"]>;
  tag?: Maybe<Scalars["String"]>;
  fee?: Maybe<Scalars["Int"]>;
  ogpImg?: Maybe<Scalars["String"]>;
  startAt?: Maybe<Scalars["NaiveDateTime"]>;
  endAt?: Maybe<Scalars["NaiveDateTime"]>;
  creatorId?: Maybe<Scalars["Int"]>;
  published?: Maybe<Scalars["Boolean"]>;
  memo?: Maybe<Scalars["String"]>;
};

export type QueryAdmin = {
  apiVersion: Scalars["String"];
  me?: Maybe<User>;
  event: Event;
  events: Array<Event>;
  contact: Contact;
  contacts: Array<Contact>;
};

export type QueryAdminEventArgs = {
  id: Scalars["Int"];
};

export type QueryAdminEventsArgs = {
  by?: Maybe<EventQuery>;
};

export type QueryAdminContactArgs = {
  id: Scalars["Int"];
};

export type QueryAdminContactsArgs = {
  by?: Maybe<ContactQuery>;
};

export type User = {
  uid: Scalars["String"];
};

export type MutationAdmin = {
  createEvent: Event;
  updateEvent: Event;
  deleteEvent: Event;
  updateContact: Contact;
  deleteContact: Contact;
};

export type MutationAdminCreateEventArgs = {
  event: EventInput;
};

export type MutationAdminUpdateEventArgs = {
  id: Scalars["Int"];
  update: EventUpdate;
};

export type MutationAdminDeleteEventArgs = {
  id: Scalars["Int"];
};

export type MutationAdminUpdateContactArgs = {
  id: Scalars["Int"];
  update: ContactUpdate;
};

export type MutationAdminDeleteContactArgs = {
  id: Scalars["Int"];
};
