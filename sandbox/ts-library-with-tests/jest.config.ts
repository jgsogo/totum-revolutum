import type {Config} from 'jest';

const config: Config = {
  globals: { TextDecoder, TextEncoder },
};

export default config;