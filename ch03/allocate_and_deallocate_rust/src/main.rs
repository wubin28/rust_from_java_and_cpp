use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

/// 释放内存函数
fn deallocate_memory(ptr: *mut u8, layout: Layout) {
    println!("释放内存...");
    // 释放内存
    unsafe { dealloc(ptr, layout) }
}

fn allocate_and_deallocate() {
    // 定义内存布局：分配100个i32大小的内存，并确保对齐
    let layout = Layout::from_size_align(
        100 * std::mem::size_of::<i32>(),
        std::mem::align_of::<i32>(),
    )
    .unwrap();

    // 分配堆外内存
    let ptr = unsafe { alloc(layout) };

    // 检查分配是否成功
    if ptr.is_null() {
        panic!("内存分配失败");
    }

    println!("成功分配内存，地址: {:?}", ptr);

    // 使用堆外内存：写入0到99的值
    for i in 0..100 {
        unsafe {
            // 计算i32类型的指针偏移
            let offset_ptr = (ptr as *mut i32).add(i);
            ptr::write(offset_ptr, i as i32);
        }
    }

    // 释放内存，Rust的所有权机制确保内存只被释放一次
    deallocate_memory(ptr, layout);

    println!("内存释放成功");

    // 尝试再次释放内存会在运行使导致错误，因为所有权已经被转移
    // deallocate_memory(ptr, layout);
    // 将上一行代码注释去掉，便宜不会报错，但在运行时会出现如下错误：
    // free(): double free detected in tcache 2
    // [1]    340748 IOT instruction (core dumped)  cargo run
}

fn main() {
    allocate_and_deallocate();
    println!("程序执行完毕");
}
// 运行结果：
// 成功分配内存，地址: 0x582b5319dba0
// 释放内存...
// 内存释放成功
// 程序执行完毕
