use my_lib;

#[test]
fn test_rand_even() {
    for _ in 0..100 {
        // 返り値が偶数かテスト
        let result = my_lib::rand_even();
        assert_eq!(result % 2, 0);
    }
}