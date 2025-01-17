// Represents a model that will be sent using protobuf serialization
pub trait OutgoingModel {
    fn encode_to_vec(&self) -> Vec<u8>;
}
