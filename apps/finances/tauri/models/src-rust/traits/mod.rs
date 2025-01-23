use prost::Message;

/// An object that wraps a proto message. This trait ensures that the object implements a few
/// other traits:
///
/// * [`TryFrom<Vec<u8>>`]: To create the object from a protobuf message.
/// * [`Into<TProto>`]: To extract the underlying proto from the type so it can be actually added to protobuf messages.
/// * [`From<&'a TProto>`]: To create wrapper references that can be passed around: it doesn't clone the proto, users
///   of the reference can benefit from the wrapper functionality.
///
/// TODO: Do we need these traits?
/// * [`From<TProto>`]: To convert a proto instance into this object
/// * [`AsRef<Self>`]: To pass references to other elementos
/// * [`AsRef<TProto>`]: To get a reference to the inner proto
///
pub trait ProtoWrapper<TProto: Message>:
    TryFrom<Vec<u8>, Error = crate::errors::Error> + Into<TProto> + private_parts::ProtoWrapperPrivate<TProto>
where
    for<'a> &'a Self: From<&'a TProto>,
{
    /// Encode the inner proto back into a buffer
    fn encode_to_vec(&self) -> Vec<u8> {
        self.inner_proto().encode_to_vec()
    }
}

pub(crate) mod private_parts {
    use prost::Message;

    pub trait ProtoWrapperPrivate<TProto: Message> {
        fn inner_proto(&self) -> &TProto;
    }
}
