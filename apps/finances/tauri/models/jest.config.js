// module.exports = {
//     testEnvironment: "node",
//   };


/** @type {import('ts-jest').JestConfigWithTsJest} **/
module.exports = {
    globals: { TextDecoder, TextEncoder },
    // testMatch: ["**/*.test.js", "**/*.test.ts"],
    // preset: 'ts-jest',
    // transform: {
    //   "^.+.tsx?$": "ts-jest",
    // },
    testEnvironment: "jsdom"
};
