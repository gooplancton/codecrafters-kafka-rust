#[derive(Debug, Clone)]
pub struct TagBuffer;

pub type UUIDv4 = u128;

pub struct CompactString(pub String);

#[derive(Debug)]
pub struct CompactArray<T>(pub Vec<T>);

#[derive(Debug)]
pub struct CompactRecords;

