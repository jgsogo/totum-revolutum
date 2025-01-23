use prost::Message;

/// An object that wraps a proto message.
///
/// * [`Into<TProto>`]: To extract the underlying proto from the type so it can be actually added to protobuf messages.
///   of the reference can benefit from the wrapper functionality.
pub trait ProtoWrapper<TProto: Message + Default>: Into<TProto> + private_parts::ProtoWrapperPrivate<TProto>
where
    TProto: for<'a> From<&'a Self>,
{
    /// Creates a reference to a **non owning** reference of the wrapper
    fn new_ref(proto: &TProto) -> &Self;

    /// Encode the inner proto back into a buffer
    fn encode_to_vec(&self) -> Vec<u8> {
        self.inner_proto().encode_to_vec()
    }

    /// Decodes an instance of the message from a buffer.
    ///
    /// The entire buffer will be consumed.
    fn decode(buf: Vec<u8>) -> Result<Self, crate::errors::Error> {
        let proto = TProto::decode(&*buf)?;
        Ok(Self::from_proto(proto))
    }
}

pub(crate) mod private_parts {
    use super::Message;

    pub trait ProtoWrapperPrivate<TProto: Message> {
        fn inner_proto(&self) -> &TProto;

        fn from_proto(proto: TProto) -> Self;
    }
}
