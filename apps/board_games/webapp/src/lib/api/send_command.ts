export async function sendCommand(roomId: string, payload: Uint8Array) {
  let i = Math.floor(Math.random() * 10);
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
