# Rust Malloc Examples

related libraries:

 - [Allocators in Rust](https://github.com/ezrosent/allocators-rs)
 - [https://github.com/rustwasm/wee_alloc](https://github.com/rustwasm/wee_alloc)
 - [https://github.com/gnzlbg/jemallocator](https://github.com/gnzlbg/jemallocator)
 - [dlmalloc-rs](https://github.com/alexcrichton/dlmalloc-rs)
 - [linked-list-allocator](https://github.com/phil-opp/linked-list-allocator)

i9g:

 - [C2Rust](https://github.com/immunant/c2rust) helps you migrate C99-compliant code to Rust. The translator (or transpiler) produces unsafe Rust code that closely mirrors the input C code.
 - [bumpalo](https://github.com/fitzgen/bumpalo)

Rewrite notes:

 - [Heap Allocation](https://os.phil-opp.com/heap-allocation/)
 - [Allocator Designs](https://os.phil-opp.com/allocator-designs/)
 - [Allocation API, allocators and virtual memory](https://notes.iveselov.info/programming/allocation-api-and-allocators)
 - [Refactoring Rust Transpiled from C](https://immunant.com/blog/2020/09/transpiled_c_safety/)
 - [Examining ARM vs X86 Memory Models with Rust](https://www.nickwilcox.com/blog/arm_vs_x86_memory_model/)
 - [Rust's Runtime](https://blog.mgattozzi.dev/rusts-runtime/)

中文：

 - [分配器设计](https://sammyne.github.io/2020/07/31/blog-os-11-allocator-designs/)
 - [堆内存分配](https://sammyne.github.io/2020/08/05/blog-os-10-heap-allocation/)

## Length 240


source code: [alloc.rs](https://github.com/rust-lang/rust/blob/f844ea1e561475e6023282ef167e76bc973773ef/src/libstd/sys/unix/alloc.rs#L14)

|  OS |  RT Allocated, bytes |  Remaining in use, bytes |
|-----|----------------------|--------------------------|
| macOS | 241 | 237 |
| Windows | 441 | 173 |
| Linux | 177 | 173 |