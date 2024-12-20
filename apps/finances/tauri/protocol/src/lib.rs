// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod snazzy {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/finances_app.rs"));
    }
}
