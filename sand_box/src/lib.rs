#[no_mangle]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
#[no_mangle]
pub fn fibo(n: i32) -> i32 {
    match n {
        0 ... 1 => n,
        _ => fibo(n - 1) + fibo(n - 2)
    }
}
