use zero::ch5::rand_even;

#[test]
fn test_rand_even() {
    for _ in 0..100 {
        // 返り値が偶数かテスト
        let result = rand_even();
        assert_eq!(result % 2, 0);
    }
}