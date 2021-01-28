<template>
  <div class="h-screen">
    <router-view />
  </div>
</template>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: var(--color-brown);
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: var(--color-brown);

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
