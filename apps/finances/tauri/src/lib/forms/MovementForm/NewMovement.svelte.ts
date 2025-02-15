import { dateWrapper2Date } from "$lib/utils";
import { Decimal, Money, type CurrencyCode } from "../../../../../../../libraries/googleapis/src-js";
import { DateWrapper } from "../../../../../../../libraries/googleapis/src-js/date";
import { Account, MovementType, Snapshot, Movement as MovementModel, MoneyAmountNumerable, MoneyAmountNonNumerable } from "../../../../models/src-js";
import { FxQuote, FxQuotePair } from "../../../../models/src-js/fx_quote";
import { MovementAmount, MovementAmountDividend, MovementDirection } from "../../../../models/src-js";


export enum NewMovementType {
    NonNumerable = 'NonNumerable',
    Numerable = 'Numerable',
    Dividend = 'Dividend',
}

export class NewMovement {
    account?: Account = $state();
    snapshots?: Snapshot[] = $state();
    mov_type?: MovementType = $state();
    date_value?: Date | null = $state();
    fx?: number = $state();

    type: NewMovementType = $state(NewMovementType.NonNumerable);

    // non-numerable movement
    amount?: number = $state();

    // numerable movement
    quantity?: number = $state();
    unit_value?: number = $state();

    // dividend movement
    ex_dividend_date?: Date | null = $state();
    ex_dividend_snapshot?: Snapshot = $state();

    constructor(type: NewMovementType, account?: Account, date_value?: Date, snapshots?: Snapshot[]) {
        this.type = type;
        this.account = account;
        this.snapshots = snapshots;
        this.date_value = date_value ?? new Date();
    }

    static create_from(movement: MovementModel, account: Account): NewMovement {
        let type = NewMovementType.NonNumerable;
        if (movement.movement_amount().as_numerable()) {
            type = NewMovementType.Numerable;
        } else if (movement.movement_amount().as_non_numerable()) {
            type = NewMovementType.NonNumerable;
        } else if (movement.movement_amount().as_dividend()) {
            type = NewMovementType.Dividend;
        } else {
            throw new Error("Movement type not recognized");
        }

        let date = dateWrapper2Date(movement.date_value());
        let new_movement = new NewMovement(type, account, date);
        new_movement.mov_type = movement.type();
        if (movement.fx_quote()) {
            new_movement.fx = movement.fx_quote()?.quote().as_number();
        }
        new_movement.amount = movement.movement_amount().amount().amount();
        new_movement.quantity = movement.movement_amount().as_numerable()?.quantity().as_number();
        new_movement.unit_value = movement.movement_amount().as_numerable()?.unit_value().amount();
        let ex_dividend_date = movement.movement_amount().as_dividend()?.ex_dividend_date();
        if (ex_dividend_date) {
            new_movement.ex_dividend_date = dateWrapper2Date(ex_dividend_date);
        }
        // new_movement.ex_dividend_snapshot = movement.movement_amount().as_dividend()?.
        return new_movement;
    }

    is_valid(date_required: boolean, base_ccy: string): boolean {
        let valid = this.account instanceof Account &&
            this.mov_type instanceof MovementType &&
            (!date_required || (date_required && this.date_value instanceof Date)) &&
            (this.account.ccy() === base_ccy || this.fx != undefined);
        switch (this.type) {
            case NewMovementType.NonNumerable:
                return valid && this.amount != undefined;
            case NewMovementType.Numerable:
                return valid && this.quantity != undefined && this.unit_value != undefined;
            case NewMovementType.Dividend:
                return valid && this.ex_dividend_date instanceof Date && this.ex_dividend_snapshot instanceof Snapshot;
        }
    }

    total(base_ccy: CurrencyCode): Money | undefined {
        if (!this.account) return undefined;

        let total = 0;
        switch (this.type) {
            case NewMovementType.NonNumerable:
                if (!this.amount) return undefined;
                total = this.amount;
                break;
            case NewMovementType.Numerable:
                if (!this.quantity || !this.unit_value) return undefined;
                total = this.quantity * this.unit_value;
                break;
            case NewMovementType.Dividend:
                if (!this.ex_dividend_snapshot) return undefined;
                let amount_numerable = this.ex_dividend_snapshot.amount().as_numerable();
                if (!amount_numerable) return undefined;
                total = amount_numerable.amount().amount();
                break;
        }

        if (this.account.ccy() !== base_ccy) {
            if (!this.fx) return undefined;
            total = total / this.fx;
        }
        return Money.create_from_number(base_ccy, total);
    }


    toMessage(direction: MovementDirection, base_ccy: CurrencyCode): MovementModel {
        let date_value = DateWrapper.create_from_date(this.date_value!);

        let fx_quote = undefined;
        if (this.fx) {
            const fx = Decimal.create_from_number(this.fx);
            const fx_pair = FxQuotePair.create_from(base_ccy, this.account!.ccy());
            fx_quote = FxQuote.create_from(date_value, fx_pair, fx);
        }

        let amount: MoneyAmountNumerable | MoneyAmountNonNumerable | MovementAmountDividend;
        switch (this.type) {
            case NewMovementType.NonNumerable:
                amount = MoneyAmountNonNumerable.create_from(Money.create_from_number(this.account!.ccy(), this.amount!));
                break;
            case NewMovementType.Numerable:
                amount = MoneyAmountNumerable.create_from(Money.create_from_number(this.account!.ccy(), this.unit_value!), Decimal.create_from_number(this.quantity!));
                break;
            case NewMovementType.Dividend:
                let ex_dividend_date = DateWrapper.create_from_date(this.ex_dividend_date!);
                const payout = MoneyAmountNumerable.create_from(Money.create_from_number(this.account!.ccy(), this.unit_value!), this.ex_dividend_snapshot!.amount().as_numerable()!.quantity());
                amount = MovementAmountDividend.create_from(ex_dividend_date, payout);
                break;
        }
        let movement_amount: MovementAmount = MovementAmount.create_from(amount);

        const data = MovementModel.create_from(date_value, undefined, this.mov_type!, direction, movement_amount, this.account!, fx_quote);
        return data;
    }
};
