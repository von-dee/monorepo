use serde::{Deserialize, Serialize};
pub mod serialization;
use polywrap_wasm_rs::{BigInt, Read, Write};
pub use serialization::{
    deserialize_big_int_input, read_big_int_input, serialize_big_int_input, write_big_int_input,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BigIntInput {
    pub prop1: BigInt,
    pub prop2: Option<BigInt>,
}

impl BigIntInput {
    pub fn new() -> BigIntInput {
        BigIntInput {
            prop1: BigInt::default(),
            prop2: None,
        }
    }

    pub fn to_buffer(input: &BigIntInput) -> Vec<u8> {
        serialize_big_int_input(input)
    }

    pub fn from_buffer(input: &[u8]) -> BigIntInput {
        deserialize_big_int_input(input)
    }

    pub fn write<W: Write>(input: &BigIntInput, writer: &mut W) {
        write_big_int_input(input, writer);
    }

    pub fn read<R: Read>(reader: &mut R) -> BigIntInput {
        read_big_int_input(reader).expect("Failed to read BigIntInput")
    }
}
