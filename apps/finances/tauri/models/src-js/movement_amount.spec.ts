import { describe, expect, test } from 'vitest';

import { CurrencyCode, Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { MoneyAmountNonNumerable, MoneyAmountNumerable } from "./money_amount.js";
import { Decimal } from "../../../../../libraries/googleapis/src-js/decimal.js";
import { Decimal as DecimalJS } from "decimal.js"
import { MovementAmount, MovementAmountDividend } from './movement.js';
import { DateWrapper } from '../../../../../libraries/googleapis/src-js/date.js';

describe('MovementAmount numerable', () => {
    let unit_value = Money.create_from_number(CurrencyCode.EUR, 123);
    let quantity = Decimal.create_from_number(10);
    let money_amount_numerable = MoneyAmountNumerable.create_from(unit_value, quantity);

    let mov_amount_numerable = MovementAmount.create_from(money_amount_numerable)

    test('amount', () => {
        expect(mov_amount_numerable.amount().currency_code()).toBe(CurrencyCode.EUR);
        expect(mov_amount_numerable.amount().amount()).toBe(1230);
    });

    test('as_non_numerable', () => {
        expect(mov_amount_numerable.as_non_numerable()).toBeUndefined();
    });

    test('as_dividend', () => {
        expect(mov_amount_numerable.as_dividend()).toBeUndefined();
    });

    test('as_numerable', () => {
        let as_numerable = mov_amount_numerable.as_numerable();
        expect(as_numerable).toBeDefined();
        expect(as_numerable!.amount().currency_code()).toBe(CurrencyCode.EUR);
        expect(as_numerable!.amount().amount()).toBe(1230);
        expect(as_numerable!.quantity().as_number()).toBe(10);
        expect(as_numerable!.quantity().as_decimal()).toStrictEqual(new DecimalJS(10));
        expect(as_numerable!.unit_value().currency_code()).toBe(CurrencyCode.EUR);
        expect(as_numerable!.unit_value().amount()).toBe(123);
    });
});


describe('MovementAmount non numerable', () => {
    let amount = Money.create_from_number(CurrencyCode.USD, 123);
    let money_amount_non_numerable = MoneyAmountNonNumerable.create_from(amount);

    let mov_amount_non_numerable = MovementAmount.create_from(money_amount_non_numerable)

    test('amount', () => {
        expect(mov_amount_non_numerable.amount().currency_code()).toBe(CurrencyCode.USD);
        expect(mov_amount_non_numerable.amount().amount()).toBe(123);
    });

    test('as_numerable', () => {
        expect(mov_amount_non_numerable.as_numerable()).toBeUndefined();
    });

    test('as_dividend', () => {
        expect(mov_amount_non_numerable.as_dividend()).toBeUndefined();
    });

    test('as_non_numerable', () => {
        let as_non_numerable = mov_amount_non_numerable.as_non_numerable();
        expect(as_non_numerable).toBeDefined();
        expect(as_non_numerable!.amount().currency_code()).toBe(CurrencyCode.USD);
        expect(as_non_numerable!.amount().amount()).toBe(123);
    });
});



describe('MovementAmount dividend', () => {
    let unit_value = Money.create_from_number(CurrencyCode.EUR, 123);
    let quantity = Decimal.create_from_number(10);
    let payout = MoneyAmountNumerable.create_from(unit_value, quantity);
    let ex_dividend_date = DateWrapper.create_from_yyyy_mm_dd(2025, 2, 6);

    let movement_amount_dividend = MovementAmountDividend.create_from(ex_dividend_date, payout);
    let mov_amount_dividend = MovementAmount.create_from(movement_amount_dividend)

    test('amount', () => {
        expect(mov_amount_dividend.amount().currency_code()).toBe(CurrencyCode.EUR);
        expect(mov_amount_dividend.amount().amount()).toBe(1230);
    });

    test('as_numerable', () => {
        expect(mov_amount_dividend.as_numerable()).toBeUndefined();
    });

    test('as_non_numerable', () => {
        expect(mov_amount_dividend.as_non_numerable()).toBeUndefined();
    });

    test('as_dividend', () => {
        let as_dividend = mov_amount_dividend.as_dividend();
        expect(as_dividend).toBeDefined();
        expect(as_dividend!.ex_dividend_date().year()).toBe(2025);
        expect(as_dividend!.ex_dividend_date().month()).toBe(2);
        expect(as_dividend!.ex_dividend_date().day()).toBe(6);
        expect(as_dividend!.ex_dividend_date().toString()).toBe("2025-02-06");
        expect(as_dividend!.payout().amount().currency_code()).toBe(CurrencyCode.EUR);
        expect(as_dividend!.payout().amount().amount()).toBe(1230);
        expect(as_dividend!.payout().unit_value().amount()).toBe(123);
        expect(as_dividend!.payout().unit_value().currency_code()).toBe(CurrencyCode.EUR);
        expect(as_dividend!.payout().quantity().as_number()).toBe(10);
        expect(as_dividend!.payout().quantity().as_decimal()).toStrictEqual(new DecimalJS(10));
    });
});
