extern crate libc;

use std::alloc::{GlobalAlloc, Layout, alloc};
use std::ptr::null_mut;

// // custom malloc
// struct MyAllocator;
//
// unsafe impl GlobalAlloc for MyAllocator {
//     unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
// }
//
// #[global_allocator]
// static A: MyAllocator = MyAllocator;

// replace system malloc
// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;
// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

use std::alloc::System;

#[global_allocator]
static A: System = System;

fn main() {
    println!("Hello, world!");
}

struct NormalString {
    pub length: i32,
    pub str: String,
}

impl NormalString {
    pub fn new(str: String) -> Self {
        NormalString { length: str.clone().len() as i32, str }
    }
}

#[cfg(test)]
mod tests {
    use crate::NormalString;
    use libc::malloc;
    use std::alloc::Layout;

    #[test]
    fn should_create_short_string_with_struct_success() {
        let long_str = "Boolean";
        let utf_string = NormalString::new(String::from(long_str));
        unsafe {
            let x = malloc(utf_string.length as usize) as *mut u32;
            assert_ne!(*x, 0);
        }
    }

    #[test]
    fn should_create_long_string_with_struct_success() {
        let long_str = "\\b(AbsoluteTime|Boolean|Byte|ByteCount|ByteOffset|BytePtr|CompTimeValue|ConstLogicalAddress|ConstStrFileNameParam|ConstStringPtr|Duration|Fixed|FixedPtr|Float32|Float32Point|Float64|Float80|Float96|FourCharCode|Fract|FractPtr|Handle|ItemCount|LogicalAddress|OptionBits|OSErr|OSStatus|OSType|OSTypePtr|PhysicalAddress|ProcessSerialNumber|ProcessSerialNumberPtr|ProcHandle|Ptr|ResType|ResTypePtr|ShortFixed|ShortFixedPtr|SignedByte|SInt16|SInt32|SInt64|SInt8|Size|StrFileName|StringHandle|StringPtr|TimeBase|TimeRecord|TimeScale|TimeValue|TimeValue64|UInt16|UInt32|UInt64|UInt8|UniChar|UniCharCount|UniCharCountPtr|UniCharPtr|UnicodeScalarValue|UniversalProcHandle|UniversalProcPtr|UnsignedFixed|UnsignedFixedPtr|UnsignedWide|UTF16Char|UTF32Char|UTF8Char)\\b";
        let utf_string = NormalString::new(String::from(long_str));
        unsafe {
            let x = malloc(utf_string.length as usize) as *mut u32;
            assert_ne!(*x, 0);
        }
    }
}
