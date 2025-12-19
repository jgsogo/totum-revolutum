import { join } from "path";
import { fileURLToPath } from "url";

const fixturesDir = fileURLToPath(new URL(".", import.meta.url));

export const USA_PROTO_FILEPATH = join(fixturesDir, "data.bin")
