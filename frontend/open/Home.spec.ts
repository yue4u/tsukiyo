import { shallowMount } from "@vue/test-utils";
import Home from "./Home.vue";

describe("Home", () => {
  it.skip("renders", () => {
    const wrapper = shallowMount(Home);
    expect(wrapper.text()).toMatchSnapshot();
  });
});
