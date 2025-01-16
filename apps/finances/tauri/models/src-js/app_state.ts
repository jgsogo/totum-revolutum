import { AppState as AppStateProto, AppStateSchema } from "../protos/app_state_pb.js";
import { fromBinary } from "@bufbuild/protobuf";
import { Buffer } from 'buffer';
import { IncomingMessageConstructor, staticImplements } from "./message.js";

export class AppState {
    private readonly app_state: AppStateProto;

    constructor(app_state: AppStateProto) {
        this.app_state = app_state;
    }

    static create_from(data: ArrayBuffer): AppState {
        const context: AppStateProto = fromBinary(AppStateSchema, Buffer.from(data, 0, data.byteLength));
        return new AppState(context);
    }

    base_ccy(): string {
        return this.app_state.baseCcy;
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
