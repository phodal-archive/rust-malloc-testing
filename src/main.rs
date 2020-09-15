extern crate libc;

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