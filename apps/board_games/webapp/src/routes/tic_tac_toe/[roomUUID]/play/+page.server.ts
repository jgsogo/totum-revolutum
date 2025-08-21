import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({params, cookies}) => {
    // Get/assign a session-id
    let session_id = cookies.get('session-id');
    if (session_id === undefined) {
        session_id = crypto.randomUUID();
    }
    cookies.set('session-id', session_id, { path: '/' });

    // Add or retrieve me as a participant
    //  * if there are no seats left, print a nice message
    //  * seats are added randomly

    return {
        session_id
    };
};
