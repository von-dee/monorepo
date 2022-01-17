pub mod wrapped;
pub use wrapped::method_wrapped;
pub mod serialization;
pub use serialization::{deserialize_method_args, serialize_method_result, InputMethod};
