import { describe, test, expect } from 'vitest';
import { FxQuote, FxQuotePair } from "./fx_quote.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { CurrencyCode, Money } from "../../../../../libraries/googleapis/src-js/money.js";
import { Decimal } from "../../../../../libraries/googleapis/src-js/decimal.js";

describe('FxQuote', () => {
    let date_value = DateWrapper.create_from_yyyy_mm_dd(2025, 2, 7);
    let fx_pair = FxQuotePair.create_from(CurrencyCode.EUR, CurrencyCode.USD);
    let fx_quote = FxQuote.create_from(date_value, fx_pair, Decimal.create_from_number(0.5));

    test('members', () => {
        expect(fx_quote.date_value().toString()).toStrictEqual("2025-02-07");
        expect(fx_quote.fx_pair().base).toBe(CurrencyCode.EUR);
        expect(fx_quote.fx_pair().quote).toBe(CurrencyCode.USD);
        expect(fx_quote.quote().as_number()).toBe(0.5);
    });

    test('apply-to-base', () => {
        let base_money = Money.create_from_number(CurrencyCode.EUR, 100);
        let money = fx_quote.apply_to(base_money);
        expect(money.currency_code()).toBe(CurrencyCode.USD);
        expect(money.amount()).toBe(50);
    });

    test('apply-to-quoted', () => {
        let quote_money = Money.create_from_number(CurrencyCode.USD, 100);
        let money = fx_quote.apply_to(quote_money);
        expect(money.currency_code()).toBe(CurrencyCode.EUR);
        expect(money.amount()).toBe(200);
    });
});

describe('FxQuotePair', () => {
    let fx_pair = FxQuotePair.create_from(CurrencyCode.USD, CurrencyCode.EUR);

    test('members', () => {
        expect(fx_pair.base).toBe(CurrencyCode.USD);
        expect(fx_pair.quote).toBe(CurrencyCode.EUR);
    });

    test('toString', () => {
        expect(fx_pair.toString()).toStrictEqual("USD/EUR");
    });

    test('error ctor', () => {
        expect(() => FxQuotePair.create_from(CurrencyCode.USD, CurrencyCode.USD)).toThrowError('Base (USD) and quote (USD) currencies have to be different');
    });
});
