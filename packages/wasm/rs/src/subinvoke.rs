use crate::msgpack::{Write, WriteEncoder};
use crate::Context;
use std::convert::AsMut;

/// Subinvoke API Helper
pub fn w3_subinvoke(
    uri: &str,
    module: &str,
    method: &str,
    input: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let mut writer = WriteEncoder::new(&[], Context::new());
    let input_str_res = String::from_utf8(input);

    match input_str_res {
        Ok(s) => {
            let buf = [uri, module, method, &s].concat().as_bytes().to_vec();
            let buf_arr = convert_slice_to_array(&buf);
            let i = i32::from_be_bytes(buf_arr);
            match writer.write_i32(&i) {
                Ok(_) => Ok(writer.get_buffer()),
                Err(_) => Err(format!("EncodingError: Int32WriteError")),
            }
        }
        Err(_) => Err(format!("Subinvoke error")),
    }
}

fn convert_slice_to_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut array = A::default();
    <A as AsMut<[T]>>::as_mut(&mut array).clone_from_slice(slice);
    array
}
