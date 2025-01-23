use prost::Message;

// Represents a model that will be sent using protobuf serialization
pub trait OutgoingModel {
    fn encode_to_vec(&self) -> Vec<u8>;
}

/// An object that wraps a proto message. This trait ensures that the object implements a few
/// other traits:
///
/// * [`TryFrom<Vec<u8>>`]: To create the object from a protobuf message.
/// * [`Into<TProto>`]: To extract the underlying proto from the type so it can be actually added to protobuf messages.
/// * [`From<&'a TProto>`]: To create wrapper references that can be passed around so: we don't clone the proto, users
///   of the reference can benefit from the wrapper.
///
/// * [`From<TProto>`]: To convert a proto instance into this object
/// * [`AsRef<Self>`]: To pass references to other elementos
/// * [`AsRef<TProto>`]: To get a reference to the inner proto
///
pub trait ProtoWrapper<TProto: Message>: TryFrom<Vec<u8>, Error = crate::errors::Error> + Into<TProto>
where
    for<'a> &'a Self: From<&'a TProto>,
{
    /// Encode the inner proto back into a buffer
    fn encode_to_vec(&self) -> Vec<u8>;
}

// /// On top of [`ProtoWrapper<TProto>`], this object implements `as_ref` function to return an
// /// object that contains a reference to the proto.
// ///
// /// This will be needed if this proto is a component of another one.
// ///
// ///  * [`Deref<Target = Self::Reference<'a>>`]: To create the object from a protobuf message
// ///  * [`Into<TProto>`]: To convert the object into the proto instance
// ///  * [`From<TProto>`]: To convert a proto instance into this object
// ///  * [`AsRef<TProto>`]: To get a reference to the inner proto
// ///
// pub trait ProtoWrapperWithRef<'a, TProto: 'a + Message>:
//     ProtoWrapper<TProto>
//     // + AsRef
//     + Deref<Target = Self::Reference>
// where
//     Self: 'a,
// {
//     type Reference: ProtoWrapperRef<TProto>;

//     fn as_ref(&'a self) -> &'a Self::Reference;
// }

// /// A wrapper over a reference to a proto
// ///
// /// * [`Into<TProto>`]: So we can get the proto (a clone)
// pub trait ProtoWrapperRef<TProto: Message>: Into<TProto> + AsRef<Self> {
//     fn new(proto: &TProto) -> &Self;

//     // fn as_proto(&self) -> &TProto;
// }
