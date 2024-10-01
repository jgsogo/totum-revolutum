import type { PageLoad } from './$types';
import {Account} from '$lib/models/Account';

export const load: PageLoad = async ({ params }) => {
    const account = await Account.Create(Number(params.pk));
    return { account };
};
