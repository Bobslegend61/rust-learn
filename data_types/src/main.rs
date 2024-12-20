fn main() {
    // - Data types are divided into 2
    // - Scalar and Compound types
        // - SCALAR
        // - Includes integers, floating point numbers, booleans, and characters
            // - INTEGERS - Numbers without fractions
            // Length	    Signed	Unsigned
            // 8-bit	    i8	      u8
            // 16-bit	    i16	      u16
            // 32-bit	    i32	      u32
            // 64-bit	    i64	      u64
            // 128-bit	    i128	  u128
            // arch	        isize     usize

            // -(2^(n - 1)) to 2^(n - 1) - 1
            // So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2^n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255
}
