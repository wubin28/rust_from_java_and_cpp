// 导入内存分配和释放函数，以及布局模块
use std::alloc::{alloc, dealloc, Layout};
// 导入指针模块
use std::ptr;

// 释放内存的函数
fn deallocate_memory(ptr: &mut *mut u8, layout: Layout) {
    // 检查指针是否为空
    if !ptr.is_null() {
        println!("释放内存...");
        // 使用unsafe块释放内存
        unsafe { dealloc(*ptr, layout) }
        // 将指针设置为空
        *ptr = std::ptr::null_mut();
    }
}

// 分配和释放内存的函数
fn allocate_and_deallocate() {
    // 定义内存布局，分配100个i32类型的空间
    let layout = Layout::from_size_align(
        100 * std::mem::size_of::<i32>(),
        std::mem::align_of::<i32>(),
    )
    .unwrap();

    // 使用unsafe块分配内存
    let mut ptr = unsafe { alloc(layout) };

    // 检查内存是否分配成功
    if ptr.is_null() {
        panic!("内存分配失败");
    }

    println!("成功分配内存，地址: {:?}", ptr);

    // 循环写入数据到分配的内存
    for i in 0..100 {
        unsafe {
            // 计算偏移指针
            let offset_ptr = (ptr as *mut i32).add(i);
            // 向偏移指针位置写入数据
            ptr::write(offset_ptr, i as i32);
        }
    }

    // 释放内存
    deallocate_memory(&mut ptr, layout);
    println!("内存释放成功");

    // 再次调用释放内存函数，不会导致双重释放
    deallocate_memory(&mut ptr, layout);
}

// 主函数，程序入口点
fn main() {
    // 调用分配和释放内存的函数
    allocate_and_deallocate();
    println!("程序执行完毕");
}

// 运行结果：
// 成功分配内存，地址: 0x5587a396eba0
// 释放内存...
// 内存释放成功
// 程序执行完毕
