import { AppState as AppStateProto, AppStateSchema, DatabaseConnection as DatabaseConnectionProto } from "../protos/app_state_pb.js";
import { fromBinary } from "@bufbuild/protobuf";
import { Buffer } from 'buffer';
import { IncomingMessageConstructor, staticImplements } from "./message.js";
import { CurrencyCode, currency_code_from_str } from "../../../../../libraries/googleapis/src-js/index.js";
import { Holder } from "./holder.js";
import { FxQuote, FxQuotePair } from "./fx_quote.js";

export class DatabaseConnection {
    private readonly proto: DatabaseConnectionProto;

    constructor(proto: DatabaseConnectionProto) {
        this.proto = proto;
    }

    user(): string {
        return this.proto.user;
    }

    password(): string {
        return this.proto.password;
    }

    host(): string {
        return this.proto.host;
    }

    port(): number {
        return this.proto.port;
    }

    dbname(): string {
        return this.proto.dbname;
    }

    postgres_url(): string {
        return `postgres://${this.user()}:${this.password()}@${this.host()}:${this.port()}/${this.dbname()}`;
    }
}

export class AppState {
    private readonly app_state: AppStateProto;

    constructor(app_state: AppStateProto) {
        this.app_state = app_state;
    }

    static create_from_array(data: ArrayBuffer): AppState {
        const context: AppStateProto = fromBinary(AppStateSchema, Buffer.from(data, 0, data.byteLength));
        return new AppState(context);
    }

    base_ccy(): CurrencyCode {
        return currency_code_from_str(this.app_state.baseCcy)!;
    }

    base_media_url(): string {
        return this.app_state.baseMediaUrl;
    }

    base_static_url(): string {
        return this.app_state.baseStaticUrl;
    }

    base_url(): string {
        return this.app_state.baseUrl;
    }

    holder(): Holder | undefined {
        return this.app_state.holder ? new Holder(this.app_state.holder) : undefined;
    }

    fx_spot(pair: FxQuotePair): FxQuote | undefined {
        let found = this.app_state.fxSpots.find((v) => {
            const fx_quote = new FxQuote(v);
            return (fx_quote.fx_pair().base === pair.base && fx_quote.fx_pair().quote === pair.quote)
        });
        return found ? new FxQuote(found) : undefined
    }
}
staticImplements<IncomingMessageConstructor<AppState>>(AppState);
