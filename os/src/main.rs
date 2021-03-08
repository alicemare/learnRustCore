//! 全局属性
//! - `#![no_std]`
//! 禁止调用标准库

#! [no_std]

//! 不使用`main`函数灯全部Rust-level 入口点作为程序入口

#! [no_main]

use core::panic::PanicInfo;

/// 当panic时会调用此函数
#[panic_handler]
fn panic(_info: &PanicInfo)->!{ //！：返回 Never 类型
    loop {} //暂时实现为一个死循环
}

/// 覆盖运行时系统的i一个入口crt0中的_start函数
#[no_mangle]
pub extern "C" fn _start() -> !{
    loop{}
}

