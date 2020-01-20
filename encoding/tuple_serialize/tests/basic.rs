use tuple_serialize::TupleSerialize;

#[derive(TupleSerialize)]
pub struct Serialize {
    executable: String,
    integer: u8,
    tuple: (u8, String),
}

fn main() {}
