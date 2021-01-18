import firebase from "firebase/app";
import "firebase/auth";
// import "@firebase/auth/dist/auth.esm.js";

const { VITE_APIKEY, VITE_AUTHDOMAIN } = import.meta.env;

export type User = firebase.User;

export const app = firebase.initializeApp({
  apiKey: VITE_APIKEY,
  authDomain: VITE_AUTHDOMAIN,
});
