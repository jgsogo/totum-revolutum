import { describe, expect, test } from 'vitest';

import { CurrencyCode, Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { MoneyAmountNumerable } from "./money_amount.js";
import { Decimal } from "../../../../../libraries/googleapis/src-js/decimal.js";
import { Movement, MovementAmount, MovementAmountDividend, MovementDirection } from './movement.js';
import { DateWrapper } from '../../../../../libraries/googleapis/src-js/date.js';
import { MovementType } from './movement_type.js';
import { Account, AccountCategory, AccountType } from './account.js';
import { FxQuote, FxQuotePair } from './fx_quote.js';
import { create } from '@bufbuild/protobuf';
import { MovementTypeSchema } from '../protos/movement_pb.js';
import { AccountSchema, AccountTypeSchema } from '../protos/account_pb.js';
import { CustodianSchema } from '../protos/custodian_pb.js';
import { Custodian } from './custodian.js';

function get_movement_type(): MovementType {
    let proto = create(MovementTypeSchema, { pk: BigInt(0), name: 'mov-type', breadcrumb: ["a", "b"] });
    return new MovementType(proto);
}

function get_custodian(): Custodian {
    let proto = create(CustodianSchema, { pk: BigInt(0), name: 'custodian-name' })
    return new Custodian(proto)
}


function get_account_type(): AccountType {
    let proto = create(AccountTypeSchema, { pk: BigInt(0), name: 'acctype-name', category: AccountCategory.Investment })
    return new AccountType(proto)
}

function get_account(): Account {
    let open = DateWrapper.create_from_yyyy_mm_dd(2025, 2, 7);
    let proto = create(AccountSchema, {
        pk: BigInt(0),
        name: 'account-name',
        custodian: get_custodian().as_proto(),
        type: get_account_type().as_proto(),
        currencyCode: CurrencyCode.USD.toString(),
        open: open.as_proto(),
        holderOwnsMoney: false,
        isNumerable: true,
        lastSnapshot: undefined,
    });
    return new Account(proto);
}


describe('Movement dividend', () => {
    let unit_value = Money.create_from_number(CurrencyCode.USD, 123);
    let quantity = Decimal.create_from_number(10);
    let payout = MoneyAmountNumerable.create_from(unit_value, quantity);
    let ex_dividend_date = DateWrapper.create_from_yyyy_mm_dd(2025, 2, 6);

    let movement_amount_dividend = MovementAmountDividend.create_from(ex_dividend_date, payout);
    let mov_amount_dividend = MovementAmount.create_from(movement_amount_dividend)

    let date_value = DateWrapper.create_from_yyyy_mm_dd(2025, 2, 6);
    let transaction_pk = undefined;
    let movement_type = get_movement_type();
    let fx_pair = FxQuotePair.create_from(CurrencyCode.EUR, CurrencyCode.USD);
    let fx_quote = FxQuote.create_from(date_value, fx_pair, Decimal.create_from_number(0.5));

    let account: Account = get_account();
    let movement = Movement.create_from(date_value, transaction_pk, movement_type, MovementDirection.In, mov_amount_dividend, account, fx_quote);

    test('members', () => {
        expect(movement.pk()).toBeUndefined();
        expect(movement.date_value().toString()).toStrictEqual("2025-02-06");
        expect(movement.transaction_pk()).toBeUndefined();
        expect(movement.type().name()).toBe('mov-type');
        expect(movement.direction()).toBe(MovementDirection.In);
        expect(movement.account_pk()).toBe(0);
        expect(movement.fx_quote()!.date_value().toString()).toStrictEqual("2025-02-06");
        expect(movement.fx_quote()!.fx_pair().base).toBe(CurrencyCode.EUR);
        expect(movement.fx_quote()!.fx_pair().quote).toBe(CurrencyCode.USD);
        expect(movement.fx_quote()!.quote().as_number()).toBe(0.5);
    });

    test('amount', () => {
        // The amount is always in base CCY
        let amount = movement.amount();
        expect(amount.amount()).toBe(2460);
        expect(amount.currency_code()).toBe(CurrencyCode.EUR);

    });
    test('money_amount', () => {
        let money_amount = movement.movement_amount();
        expect(money_amount.amount().amount()).toBe(1230);
        expect(money_amount.amount().currency_code()).toBe(CurrencyCode.USD);
        expect(money_amount.as_dividend()).toBeDefined();
        expect(money_amount.as_non_numerable()).toBeUndefined();
        expect(money_amount.as_numerable()).toBeUndefined();
    });


});
