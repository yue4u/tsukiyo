module.exports = {
  preset: "@vue/cli-plugin-unit-jest/presets/typescript",
  transform: {
    "^.+\\.vue$": "vue-jest",
    ".+\\.(css|styl|less|sass|scss|png|jpg|ttf|woff|woff2)$":
      "jest-transform-stub",
    "^.+\\.(js|jsx)?$": "babel-jest",
  },
  moduleNameMapper: {
    "^@/(.*)$": "<rootDir>/$1",
  },
  snapshotSerializers: ["jest-serializer-vue"],
  testMatch: ["<rootDir>/(**/*.spec.[jt]s?(x)|**/__tests__/*.(js|jsx|ts|tsx))"],
  transformIgnorePatterns: ["<rootDir>/node_modules/"],
};
