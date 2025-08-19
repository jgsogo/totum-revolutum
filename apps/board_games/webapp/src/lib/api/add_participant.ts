export async function addParticipant(roomUUID: string, participantUUID: string, participantRole: string) {
    const res = await fetch(`/room/${roomUUID}/add_participant`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/octet-stream'
        },
        body: JSON.stringify({ participantUUID, participantRole })
    });

    if (!res.ok) throw new Error('Add participant failed');

    return res.json()
}
