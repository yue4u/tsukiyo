import firebase from "firebase/app";
import { ref } from "vue";
import "firebase/auth";
import "firebase/storage";
import { authExchange as createAuthExchange } from "@urql/exchange-auth";
import { makeOperation } from "@urql/vue";
import router from "@/router";

const { VITE_API_KEY, VITE_AUTH_DOMAIN } = import.meta.env;

export type User = firebase.User;

export const app = firebase.initializeApp({
  apiKey: VITE_API_KEY,
  authDomain: VITE_AUTH_DOMAIN,
});

export const currentUser = ref<User | null>(null);

app.auth().onAuthStateChanged((maybeUser) => {
  currentUser.value = maybeUser;
});

export async function logout() {
  currentUser.value = null;
  await firebase.auth().signOut();
  router.push("/admin/login");
}
/**
 * we need this to check auth before handle gql api
 */
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
    await waitForAuth();
    return currentUser.value?.getIdToken();
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
