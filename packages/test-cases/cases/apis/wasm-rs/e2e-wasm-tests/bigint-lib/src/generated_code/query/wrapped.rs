use crate::{deserialize_method_args, method, serialize_method_result, InputMethod};

pub fn method_wrapped(input: &[u8]) -> Vec<u8> {
    let args = deserialize_method_args(input).expect("Failed to deserialize buffer");
    let result = method(InputMethod {
        arg1: args.arg1,
        arg2: args.arg2,
        obj: args.obj,
    });
    serialize_method_result(&result)
}
