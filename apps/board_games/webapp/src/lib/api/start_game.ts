export async function startGame(roomUUID: string, gameType: string) {
    const res = await fetch(`/room/${roomUUID}/_commands/start_game`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/octet-stream'
        },
        body: JSON.stringify({ gameType })
    });

    if (!res.ok) throw new Error('Start game failed');

    return res.json()
}
