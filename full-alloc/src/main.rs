use std::alloc::{GlobalAlloc, Layout};
use std::cmp;
use std::ptr;
use libc;

struct System;

const MIN_ALIGN: usize = 16;

unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            libc::malloc(layout.size()) as *mut u8
        } else {
            #[cfg(target_os = "macos")]
                {
                    if layout.align() > (1 << 31) {
                        return ptr::null_mut()
                    }
                }
            aligned_malloc(&layout)
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut libc::c_void)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            libc::calloc(layout.size(), 1) as *mut u8
        } else {
            let ptr = self.alloc(layout.clone());
            if !ptr.is_null() {
                ptr::write_bytes(ptr, 0, layout.size());
            }
            ptr
        }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
            libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
        } else {
            realloc_fallback(self, ptr, layout, new_size)
        }
    }
}

unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
    let mut out = ptr::null_mut();
    let ret = libc::posix_memalign(&mut out, layout.align(), layout.size());
    if ret != 0 {
        ptr::null_mut()
    } else {
        out as *mut u8
    }
}

unsafe fn realloc_fallback(
    alloc: &System,
    ptr: *mut u8,
    old_layout: Layout,
    new_size: usize,
) -> *mut u8 {
    // Docs for GlobalAlloc::realloc require this to be valid:
    let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

    let new_ptr = GlobalAlloc::alloc(alloc, new_layout);
    if !new_ptr.is_null() {
        let size = cmp::min(old_layout.size(), new_size);
        ptr::copy_nonoverlapping(ptr, new_ptr, size);
        GlobalAlloc::dealloc(alloc, ptr, old_layout);
    }
    new_ptr
}

#[global_allocator]
static HEAP: System = System;

struct NormalString {
    pub length: i32,
    pub str: String,
}

impl NormalString {
    pub fn new(str: String) -> Self {
        NormalString { length: str.clone().len() as i32, str }
    }
}

fn main() {
    println!("Hello, world!");
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

