import * as db from '$lib/server/database';
import type { ServerInit } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

export const init: ServerInit = async () => {
    const connectionString = `postgresql://${env.BOARD_GAMES_WEBAPP_SQL_USER}:${env.BOARD_GAMES_WEBAPP_SQL_PASSWORD}@${env.BOARD_GAMES_WEBAPP_SQL_HOST}:${env.BOARD_GAMES_WEBAPP_SQL_PORT}/${env.BOARD_GAMES_WEBAPP_SQL_DATABASE}`
	await db.connect(connectionString);
};
