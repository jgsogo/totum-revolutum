export async function sendCommand(roomId: string, payload: Uint8Array) {
  const res = await fetch(`/room/${roomId}/command`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/octet-stream'
    },
    body: payload
  });

  if (!res.ok) throw new Error('Command failed');

  return res.json()
}
