// 使用 jemallocator 库中的 Jemalloc 内存分配器
// 要运行代码，还需要在Cargo.toml文件里加上这三行：
// [dependencies]
// jemallocator = "0.5.4"
// jemalloc-ctl = "0.5"
use jemallocator::Jemalloc;

// 用属性(用于为代码的特定部分提供元信息的注释)定义一个全局的内存分配器，使用 Jemalloc 作为系统的全局内存分配器
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

// 自定义一个包含大字符串的结构体，并实现 Clone 和 Drop trait
#[derive(Clone)]
struct LargeStringOwner {
    // 包含一个字符串字段，但允许未使用（避免编译器警告）
    #[allow(dead_code)]
    content: String,
}

impl LargeStringOwner {
    // 为结构体实现一个新的构造函数，接受字符串大小作为参数
    fn new(size: usize) -> Self {
        // 创建一个大的字符串并初始化结构体
        LargeStringOwner {
            content: create_large_string(size),
        }
    }
}

// 实现 Drop trait，添加销毁时的消息打印
impl Drop for LargeStringOwner {
    // 在结构体销毁时打印消息
    fn drop(&mut self) {
        println!("Dropping LargeStringOwner, releasing large string memory.");
    }
}

// 创建一个大的字符串函数
fn create_large_string(size: usize) -> String {
    // 创建一个具有预设容量的字符串，容量为 size
    let mut s = String::with_capacity(size);
    // 扩展字符串，填充 size 个 'A' 字符
    s.extend(std::iter::repeat('A').take(size));
    // 返回这个大字符串
    s
}

// 获取当前内存使用情况的函数
fn get_memory_usage() -> u64 {
    // 引入 jemalloc_ctl 库中的 epoch 和 stats 模块。Rust 可以在函数定义的内部使用 use 语句引入外部模块
    use jemalloc_ctl::{epoch, stats};
    // 获取 epoch 模块的 MIB（管理信息块）
    let e = epoch::mib().unwrap();
    // 获取 stats 模块的 allocated MIB
    let allocated = stats::allocated::mib().unwrap();

    // 刷新 jemalloc 的统计信息，使得获取的内存使用情况是最新的
    e.advance().unwrap();

    // 读取当前分配的内存量，单位是字节
    let allocated_bytes: u64 = (allocated.read().unwrap() / 1024).try_into().unwrap();
    // 将字节转换为 KB 并返回
    allocated_bytes
}

// 主函数，从这里开始执行程序
fn main() {
    // 获取当前系统的初始内存使用情况
    let initial_memory = get_memory_usage();
    // 打印初始内存使用情况，单位是 KB
    println!("Initial memory usage: {} KB", initial_memory);

    {
        // 进入一个新的作用域，作用域是用大括号 `{}` 包围的代码块
        let memory_before = get_memory_usage();
        // 打印创建字符串前的内存使用情况
        println!("Memory before creating String: {} KB", memory_before);

        // 创建一个包含 100M 大字符串的自定义结构体
        let large_string_owner = LargeStringOwner::new(100_000_000); // 100 MB

        // 深度克隆该结构体
        let cloned_string_owner = large_string_owner.clone();

        // 获取创建大字符串后的内存使用情况
        let memory_after = get_memory_usage();
        // 打印创建大字符串后的内存使用情况
        println!(
            "Memory after creating and cloning String: {} KB",
            memory_after
        );

        // 使用标准库的断言宏 assert!，验证内存是否增加，否则中止程序，并打印错误信息
        assert!(memory_after > memory_before);

        // 使用cloned_string_owner和large_string_owner确保内存不会在此处被释放
        println!(
            "Using large_string_owner and cloned_string_owner: {}",
            large_string_owner.content.len() + cloned_string_owner.content.len()
        );
    } // 这里作用域结束，`large_string_owner`和`cloned_string_owner`变量自动销毁，内存应该被释放

    // 获取离开作用域后的内存使用情况
    let final_memory = get_memory_usage();
    // 打印离开作用域后的内存使用情况
    println!("Memory after Strings are out of scope: {} KB", final_memory);

    // 验证最终的内存使用是否接近初始值，允许有一些小波动
    assert!(final_memory <= initial_memory + 1_000); // 容许一点点波动
}
// 运行结果:
// Initial memory usage: 33 KB
// Memory before creating String: 43 KB
// Memory after creating and cloning String: 196651 KB
// Using large_string_owner and cloned_string_owner: 200000000
// Dropping LargeStringOwner, releasing large string memory.
// Dropping LargeStringOwner, releasing large string memory.
// Memory after Strings are out of scope: 43 KB
