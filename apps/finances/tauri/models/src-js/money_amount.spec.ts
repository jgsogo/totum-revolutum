import { describe, expect, test } from 'vitest';

import { CurrencyCode, Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { MoneyAmount, MoneyAmountNonNumerable, MoneyAmountNumerable } from "./money_amount.js";
import { Decimal } from "../../../../../libraries/googleapis/src-js/decimal.js";
import { Decimal as DecimalJS } from "decimal.js"

describe('MoneyAmount numerable', () => {
    let unit_value = Money.create_from_number(CurrencyCode.EUR, 123);
    let quantity = Decimal.create_from_number(10);
    let money_amount_numerable = MoneyAmountNumerable.create_from(unit_value, quantity);

    let money_amount = MoneyAmount.create_from(money_amount_numerable);

    test('amount', () => {
        expect(money_amount.amount().currency_code()).toBe(CurrencyCode.EUR);
        expect(money_amount.amount().amount()).toBe(1230);
    });

    test('as_non_numerable', () => {
        expect(money_amount.as_non_numerable()).toBeUndefined();
    });

    test('as_numerable', () => {
        let as_numerable = money_amount.as_numerable();
        expect(as_numerable).toBeDefined();
        expect(as_numerable!.amount().currency_code()).toBe(CurrencyCode.EUR);
        expect(as_numerable!.amount().amount()).toBe(1230);
        expect(as_numerable!.quantity().as_number()).toBe(10);
        expect(as_numerable!.quantity().as_decimal()).toStrictEqual(new DecimalJS(10));
        expect(as_numerable!.unit_value().currency_code()).toBe(CurrencyCode.EUR);
        expect(as_numerable!.unit_value().amount()).toBe(123);
    });
});


describe('MoneyAmount non numerable', () => {
    let amount = Money.create_from_number(CurrencyCode.USD, 123);
    let money_amount_numerable = MoneyAmountNonNumerable.create_from(amount);

    let money_amount = MoneyAmount.create_from(money_amount_numerable);

    test('amount', () => {
        expect(money_amount.amount().currency_code()).toBe(CurrencyCode.USD);
        expect(money_amount.amount().amount()).toBe(123);
    });

    test('as_numerable', () => {
        expect(money_amount.as_numerable()).toBeUndefined();
    });

    test('as_non_numerable', () => {
        let as_non_numerable = money_amount.as_non_numerable();
        expect(as_non_numerable).toBeDefined();
        expect(as_non_numerable!.amount().currency_code()).toBe(CurrencyCode.USD);
        expect(as_non_numerable!.amount().amount()).toBe(123);
    });
});
