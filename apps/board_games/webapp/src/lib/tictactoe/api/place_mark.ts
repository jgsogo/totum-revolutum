export async function placeMark(roomUUID: string, player_session: string, cell_id: number) {
    const res = await fetch(`/tic_tac_toe/${roomUUID}/_commands/place_mark`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/octet-stream'
        },
        body: JSON.stringify({ player_session, cell_id })
    });

    if (!res.ok) throw new Error('Place mark failed');

    return res.json()
}
