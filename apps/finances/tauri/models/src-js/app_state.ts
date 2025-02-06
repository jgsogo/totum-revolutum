import { AppState as AppStateProto, AppStateSchema, DatabaseConnection as DatabaseConnectionProto } from "../protos/app_state_pb.js";
import { fromBinary } from "@bufbuild/protobuf";
import { Buffer } from 'buffer';
import { IncomingMessageConstructor, staticImplements } from "./message.js";
import { CurrencyCode, currency_code_from_str } from "../../../../../libraries/googleapis/src-js/index.js";

export class DatabaseConnection {
    private readonly proto: DatabaseConnectionProto;

    constructor(proto: DatabaseConnectionProto) {
        this.proto = proto;
    }

    postgres_url(): string {
        return this.proto.postgresUrl;
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
}
staticImplements<IncomingMessageConstructor<AppState>>(AppState);
