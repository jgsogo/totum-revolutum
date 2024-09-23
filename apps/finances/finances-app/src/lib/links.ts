// Navigation Sitemap

// TODO: These links should be retrieved from the database (based on some categories)

export type List = Array<{ href: string; label: string; keywords: string; badge?: string }>;
export const menuNavLinks: Record<string, Array<{ title: string; list: List }>> = {
	'/all': [
        // TODO: Retrieve ALL the accounts grouped by "AccountType", or grouped by holder?
		{
			title: 'ING',
			list: [
				{ href: '/all/accounts/ing1', label: 'Cuenta nómina', keywords: 'ing, account' },
				{ href: '/all/accounts/ing2', label: 'Cuenta naranja', keywords: 'ing, account' }
			]
		},
		{
			title: 'BBVA',
			list: [
				{ href: '/all/accounts/bbva/depo1', label: 'Cuenta corriente', keywords: 'myinvestor, deposit' }
			]
		},
		{
			title: 'Indexa',
			list: [
				{ href: '/all/accounts/bbva/depo1', label: 'Cuenta corriente', keywords: 'myinvestor, deposit' }
			]
		},
		{
			title: 'MyInvestor',
			list: [
				{ href: '/all/accounts/bbva/depo1', label: 'Cuenta corriente', keywords: 'myinvestor, deposit' }
			]
		}
	],
	'/accounts': [
        // TODO: Retrieve ALL the accounts grouped by "AccountType", or grouped by holder?
		{
			title: 'Checking account',
			list: [
				{ href: '/accounts/ing1', label: 'ING - Cuenta nómina', keywords: 'ing, account' },
				{ href: '/accounts/ing2', label: 'ING - Cuenta naranja', keywords: 'ing, account' }
			]
		},
		{
			title: 'Deposits',
			list: [
				{ href: '/accounts/myinvestor/depo1', label: 'MyInvestor 12M', keywords: 'myinvestor, deposit' }
			]
		}
	],
	'/investments': [
        // TODO: Retrieve ALL the accounts grouped by "AccountType"
		{
			title: 'Funds',
			list: [
				{ href: '/investments/funds/indexa', label: 'Indexa fondo', keywords: 'indexa, fund' },
				{ href: '/investments/funds/baelo', label: 'Baelo patrimonio', keywords: 'myinvestor, fund' }
			]
		},
		{
			title: 'Stocks',
			list: [
				{ href: '/investments/stocks/3m', label: 'MMM', keywords: 'degiro, mmm' },
                { href: '/investments/stocks/msft', label: 'Microsoft', keywords: 'degiro, msft' }
			]
		},
		{
			title: 'Pension plan',
			list: [
				{ href: '/investments/pension/indexa', label: 'Indexa pensiones', keywords: 'degiro, mmm' },
                { href: '/investments/pension/caixa-vida', label: 'Vida Ahorro (Caixa)', keywords: 'degiro, msft' }
			]
		}
	],
	'/rentals': [
        // TODO: Retrieve ALL the accounts grouped by "AccountType"
		{
			title: 'Houses',
			list: [
				{ href: '/rentals/house/lope-de-haro', label: 'Lope de Haro', keywords: 'indexa, fund' },
                { href: '/rentals/house/atyka', label: 'Atyka', keywords: 'myinvestor, fund' }
			]
		},
		{
			title: 'Garages',
			list: [
				{ href: '/rentals/garages/molinos', label: 'Molinos', keywords: 'degiro, mmm' }
			]
		}
	],
	'/taxes': [
		{
			title: 'IRPF',
			list: [
				{ href: '/taxes/irpf', label: 'Declaración IRPF', keywords: 'body, scroll, scrollbar, hr, horizontal, rule, divider' }
			]
		}
	]
};
