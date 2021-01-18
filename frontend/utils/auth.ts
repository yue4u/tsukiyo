import firebase from "firebase/app";
import "firebase/auth";
// import "@firebase/auth/dist/auth.esm.js";
import { authExchange as createAuthExchange } from "@urql/exchange-auth";
import { makeOperation } from "@urql/vue";
const { VITE_APIKEY, VITE_AUTHDOMAIN } = import.meta.env;

export type User = firebase.User;

export const app = firebase.initializeApp({
  apiKey: VITE_APIKEY,
  authDomain: VITE_AUTHDOMAIN,
});

export function waitForAuth(): Promise<User | null> {
  return new Promise((resolve) => {
    app.auth().onAuthStateChanged((maybeUser) => {
      if (maybeUser) {
        resolve(maybeUser);
      } else {
        resolve(null);
      }
    });
  });
}

export const authExchange = createAuthExchange<string | undefined>({
  async getAuth({ authState }) {
    if (authState) {
      return null;
    }
    const maybeUser = await waitForAuth();
    return maybeUser?.getIdToken();
  },
  // TODO: fix this type
  // @ts-ignore
  addAuthToOperation({ authState, operation }) {
    if (!authState) {
      return operation;
    }

    const fetchOptions =
      typeof operation.context.fetchOptions === "function"
        ? operation.context.fetchOptions()
        : operation.context.fetchOptions || {};
    // maybe we can use immer to simplify this
    return makeOperation(operation.kind, operation, {
      ...operation.context,
      fetchOptions: {
        ...fetchOptions,
        headers: {
          ...fetchOptions.headers,
          Authorization: `Bearer ${authState}`,
        },
      },
    });
  },
});
