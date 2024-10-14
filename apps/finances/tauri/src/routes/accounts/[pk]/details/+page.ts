import type {PageLoad} from './$types';
import {account_detail} from '$lib/commands';
import {Account} from '$lib/models/Account';

export const load: PageLoad = async ({params}) => {
    const account: Account = await account_detail(Number(params.pk));
    return {account};
};
