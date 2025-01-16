use prost::Message;

// Represents a model that will be sent using protobuf serialization
pub trait OutgoingModel {
    fn as_message(&self) -> &impl Message;
}
