use crate::utils::aliases::*;
use getset::{Getters, Setters};

// All the primitive types in rust
// We create aliases of the underling types
#[derive(Default, Getters, Setters)]
pub struct TypeExample {

    #[getset(get = "pub", set = "pub")]
    boolean: bool,

    #[getset(get = "pub", set = "pub")]
    byte: Byte, // Unsigned 8-bit integer

    #[getset(get = "pub", set = "pub")]
    s_byte: Sbyte, // Signed 8-bit integer

    #[getset(get = "pub", set = "pub")]
    char: char,

    #[getset(get = "pub", set = "pub")]
    double: Double, // no built-in decimal type

    #[getset(get = "pub", set = "pub")]
    float: Float,

    #[getset(get = "pub", set = "pub")]
    int: Int,

    #[getset(get = "pub", set = "pub")]
    uint: Uint,

    #[getset(get = "pub", set = "pub")]
    long: Long,

    #[getset(get = "pub", set = "pub")]
    ulong: Ulong,

    #[getset(get = "pub", set = "pub")]
    short: Short,

    #[getset(get = "pub", set = "pub")]
    string: String,
}