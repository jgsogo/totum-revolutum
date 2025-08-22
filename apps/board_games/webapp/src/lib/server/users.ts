

type User = Map<string, any>;
let users = new Map<string, User>();



export async function getOrCreateUser(sessionid: string | undefined): Promise<User | undefined> {
    console.log(`[backend] Get user information for sessionid '${sessionid}'`);
    if (sessionid === undefined) return undefined;

    let user = users.get(sessionid);
    if (user === undefined) {
        user = new Map([["uuid", sessionid]]);
        users.set(sessionid, user);
    }
    return user;
}

export async function addToUser(sessionid: string, key: string, value: any): Promise<User> {
    let user = await getOrCreateUser(sessionid);
    user!.set(key, value);
    users.set(sessionid, user!);
    return user!;
}
