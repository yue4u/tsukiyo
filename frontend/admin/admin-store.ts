import type { Event, Contact } from "@/type/gql";
import { reactive, inject } from "vue";

type CollectionOf<T> = { [k: string]: T };

export const events = reactive<CollectionOf<Event>>({});
export const contacts = reactive<CollectionOf<Contact>>({});

export const eventsSymbol = Symbol("events");
export const contactsSymbol = Symbol("contacts");

export const useEvents = () => inject(eventsSymbol) as typeof events;
export const useContacts = () => inject(contactsSymbol) as typeof contacts;
