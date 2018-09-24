#[no_mangle]
pub fn fibo(n: i32) -> i32 {
    match n {
        0 ... 1 => n,
        _ => fibo(n - 1) + fibo(n - 2)
    }
}
