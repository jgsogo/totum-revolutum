export async function createNewRoom(roomUUID: string, name: string) {
    const res = await fetch(`/room/${roomUUID}/create_new_room`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/octet-stream'
        },
        body: JSON.stringify({ name })
    });

    if (!res.ok) throw new Error('Room create failed');

    return res.json()
}
