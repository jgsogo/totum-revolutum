
export abstract class OutgoingMessage {
    abstract toBinary(): Uint8Array;
}


// Credit: This IncomingMessage workround is based on this stackoverflow question: https://stackoverflow.com/questions/65846848/typescript-static-methods-in-interfaces
export function staticImplements<T>(_ctor: T) { }

export interface IncomingMessageConstructor<T> {
    create_from_array(data: ArrayBuffer): T;
}
