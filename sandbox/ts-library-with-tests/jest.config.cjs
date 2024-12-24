module.exports = {
    testEnvironment: "node",
    testMatch: ["**/*.test.js"],
    preset: "ts-jest",
    transform: {
        "\\.[jt]sx?$": "ts-jest",
    }
  };