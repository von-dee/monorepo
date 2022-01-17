use polywrap_wasm_rs::{BigInt, Context, Read, ReadDecoder, Write, WriteEncoder, WriteSizer};
use serde::{Deserialize, Serialize};

use crate::BigIntInput;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputMethod {
    pub arg1: BigInt,
    pub arg2: Option<BigInt>,
    pub obj: BigIntInput,
}

pub fn deserialize_method_args(input: &[u8]) -> Result<InputMethod, String> {
    let mut context = Context::new();
    context.description = "Deserializing query-type: method".to_string();
    let mut reader = ReadDecoder::new(input, context);
    let mut num_of_fields = reader.read_map_length().unwrap();

    let mut _arg1: BigInt = BigInt::default();
    let mut _arg1_set = false;
    let mut _arg2: Option<BigInt> = None;
    let mut _obj: BigIntInput = BigIntInput::new();
    let mut _obj_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string().unwrap();

        match field.as_str() {
            "arg1" => {
                reader
                    .context()
                    .push(&field, "BigInt", "type found, reading argument");
                _arg1 = reader.read_bigint().unwrap();
                _arg1_set = true;
                reader.context().pop();
            }
            "arg2" => {
                reader
                    .context()
                    .push(&field, "Option<BigInt>", "type found, reading argument");
                _arg2 = reader.read_nullable_bigint().unwrap();
                reader.context().pop();
            }
            "obj" => {
                reader
                    .context()
                    .push(&field, "BigIntInput", "type found, reading argument");
                let object = BigIntInput::read(&mut reader);
                _obj = object;
                _obj_set = true;
                reader.context().pop();
            }
            _ => {}
        }
    }
    if !_arg1_set {
        return Err(reader
            .context()
            .print_with_context("Missing required argument: 'arg1: BigInt'"));
    }
    if !_obj_set {
        return Err(reader
            .context()
            .print_with_context("Missing required argument: 'obj: BigIntInput'"));
    }

    Ok(InputMethod {
        arg1: _arg1,
        arg2: _arg2,
        obj: _obj,
    })
}

pub fn serialize_method_result(result: &BigInt) -> Vec<u8> {
    let mut sizer_context = Context::new();
    sizer_context.description = "Serializing (sizing) query-type: method".to_string();
    let mut sizer = WriteSizer::new(sizer_context);
    write_method_result(result, &mut sizer);
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) query-type: method".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_method_result(result, &mut encoder);
    encoder.get_buffer()
}

pub fn write_method_result<W: Write>(result: &BigInt, writer: &mut W) {
    writer.context().push("method", "BigInt", "writing result");
    writer.write_bigint(result).unwrap();
    writer.context().pop();
}
