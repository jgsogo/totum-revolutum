import type { Custodian, HolderContext } from '../../../../../../../models/src-js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent }) {
    const { holder_context }: { holder_context: HolderContext } = await parent();

    let custodian_pk: number = parseInt(params.custodian_pk, 10);
    const accounts_for_custodian = holder_context.grouped_by_custodian().get(custodian_pk)!;
    const custodian: Custodian = accounts_for_custodian[0].custodian();
    return {
        accounts_for_custodian,
        custodian,
    };

}
