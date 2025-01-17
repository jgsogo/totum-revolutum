
export abstract class OutgoingMessage {
    abstract toBinary(): Uint8Array;
}


// Credit: This IncomingMessage woraround is based on this stackoverflow question: https://stackoverflow.com/questions/65846848/typescript-static-methods-in-interfaces
export function staticImplements<T>(_ctor: T) { }

export interface IncomingMessageConstructor<T> {
    create_from(data: ArrayBuffer): T;
}
