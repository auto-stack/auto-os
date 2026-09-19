// P659 cdb 烟雾探针：stack overflow / fastfail / AV / clean 四形态。
// rustc -C debuginfo=2 -O0 编译；逐形态在 cdb 编排下验证事件命令捕获。
// usage: p659_probe <so|ff|av|clean>

fn stack_overflow() {
    let pad = [0u8; 4096];
    std::hint::black_box(&pad);
    stack_overflow()
}

fn fastfail() {
    // x64 fastfail = int 29h（STATUS_STACK_BUFFER_OVERRUN 族由 __fastfail 触发）
    unsafe { std::arch::asm!("int 0x29") }
}

fn av() {
    unsafe {
        let p: *mut u8 = std::ptr::null_mut();
        std::ptr::write_volatile(p, 1);
    }
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    match mode.as_str() {
        "so" => stack_overflow(),
        "ff" => fastfail(),
        "av" => av(),
        "clean" => println!("clean exit"),
        _ => {
            eprintln!("usage: probe <so|ff|av|clean>");
            std::process::exit(2);
        }
    }
    println!("done: {mode}");
}
