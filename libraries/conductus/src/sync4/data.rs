#[derive(Clone, PartialEq, Debug)]
pub enum Message<Data> {
    Data(Data),
    Flush,
}
