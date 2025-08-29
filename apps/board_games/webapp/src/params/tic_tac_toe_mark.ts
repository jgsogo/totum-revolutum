import type { ParamMatcher } from '@sveltejs/kit';

export const match = ((param: string): param is ('X' | 'O') => {
	return param === 'X' || param === 'O'; // TODO: Move these hardcoded values to some common place
}) satisfies ParamMatcher;
