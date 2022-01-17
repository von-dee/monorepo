use std::ops::Mul;

use polywrap_wasm_rs::BigInt;
pub mod generated_code;
pub use generated_code::*;

#[no_mangle]
pub fn call_bigint_method(arg1: i64, arg2: i64, arg3: i64, arg4: i64) -> i64 {
    let bigint_input = BigIntInput {
        prop1: BigInt::from(arg1),
        prop2: Some(BigInt::from(arg2)),
    };
    let input_method = InputMethod {
        arg1: BigInt::from(arg3),
        arg2: Some(BigInt::from(arg4)),
        obj: bigint_input,
    };
    let bigint_method = method(input_method);
    bigint_method.try_into().unwrap()
}

pub fn method(input: InputMethod) -> BigInt {
    let mut result = input.arg1.mul(input.obj.prop1);

    if input.arg2.is_some() {
        result = result.mul(input.arg2.unwrap());
    }
    if input.obj.prop2.is_some() {
        result = result.mul(input.obj.prop2.unwrap());
    }

    result
}
