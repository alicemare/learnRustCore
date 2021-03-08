//! 全局属性
//! - `#![no_std]`
//! 禁止调用标准库

#! [no_std]

//! 不使用`main`函数灯全部Rust-level 入口点作为程序入口

#! [no_main]

//! 内联汇编
#![feature(llvm_asm)]

//! 内联汇编文件
#! [feature(global_asm)]


// 汇编程序编写的入口
global_asm!(include_str!("entry.asm"));

use core::panic::PanicInfo;

/// 当panic时会调用此函数
#[panic_handler]
fn panic(_info: &PanicInfo)->!{ //！：返回 Never 类型
    loop {} //暂时实现为一个死循环
}


/// 在屏幕上输出一个字符，目前我们先不用了解其实现原理
pub fn console_putchar(ch: u8) {
    let _ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        llvm_asm!("ecall"
             : "={x10}" (_ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}

/// 覆盖运行时系统的i一个入口crt0中的_start函数
#[no_mangle]
pub extern "C" fn rust_main() -> !{
    // 在屏幕上输出 "OK\n" ，随后进入死循环
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');

    loop{}
}

