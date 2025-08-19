import { Client } from 'pg';

let client: Client = undefined;

export async function connect(connectionString: string): Promise<Client> {
    console.log(`[backend] Connect to PostgreSQL using '${connectionString}'`);
    client = new Client({ connectionString });
    await client.connect();
}
