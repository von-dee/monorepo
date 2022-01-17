use crate::BigIntInput;
use polywrap_wasm_rs::{BigInt, Context, Read, ReadDecoder, Write, WriteEncoder, WriteSizer};

pub fn serialize_big_int_input(input: &BigIntInput) -> Vec<u8> {
    let mut sizer_context = Context::new();
    sizer_context.description = "Serializing (sizing) object-type: BigIntInput".to_string();
    let mut sizer = WriteSizer::new(sizer_context);
    write_big_int_input(input, &mut sizer);
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: BigIntInput".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_big_int_input(input, &mut encoder);
    encoder.get_buffer()
}

pub fn write_big_int_input<W: Write>(input: &BigIntInput, writer: &mut W) {
    writer.write_map_length(&2).unwrap();
    writer.context().push("prop1", "BigInt", "writing property");
    writer.write_str("prop1").unwrap();
    writer.write_bigint(&input.prop1).unwrap();
    writer.context().pop();
    writer
        .context()
        .push("prop2", "Option<BigInt>", "writing property");
    writer.write_str("prop2").unwrap();
    writer.write_nullable_bigint(&input.prop2).unwrap();
    writer.context().pop();
}

pub fn deserialize_big_int_input(input: &[u8]) -> BigIntInput {
    let mut context = Context::new();
    context.description = "Deserializing object-type: BigIntInput".to_string();
    let mut reader = ReadDecoder::new(input, context);
    read_big_int_input(&mut reader).expect("Failed to deserialize BigIntInput")
}

pub fn read_big_int_input<R: Read>(reader: &mut R) -> Result<BigIntInput, String> {
    let mut num_of_fields = reader.read_map_length().unwrap();

    let mut _prop1: BigInt = BigInt::default();
    let mut _prop1_set = false;
    let mut _prop2: Option<BigInt> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string().unwrap();

        match field.as_str() {
            "prop1" => {
                reader
                    .context()
                    .push(&field, "BigInt", "type found, reading property");
                _prop1 = reader.read_bigint().unwrap();
                _prop1_set = true;
                reader.context().pop();
            }
            "prop2" => {
                reader
                    .context()
                    .push(&field, "Option<BigInt>", "type found, reading property");
                _prop2 = reader.read_nullable_bigint().unwrap();
                reader.context().pop();
            }
            _ => {}
        }
    }
    if !_prop1_set {
        return Err(reader
            .context()
            .print_with_context("Missing required property: 'prop1: BigInt'"));
    }

    Ok(BigIntInput {
        prop1: _prop1,
        prop2: _prop2,
    })
}
