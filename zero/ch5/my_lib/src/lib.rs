/// 偶数の乱数を返す
pub fn rand_even() -> u32 {
    rand::random::<u32>() & !1
}