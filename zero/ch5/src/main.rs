fn main() {
    run();
}

/// 実行関数です
///
/// # Examples
///
/// ```
/// use zero::ch5::run;
/// let n = run();
/// ```
pub fn run() -> Option<u32> {
    println!("run ch5");
    let even = my_lib::rand_even();
    println!("{}", even);
    Some(100)
}

pub fn f() -> Option<u32> {
    Some(100)
}

pub fn pred(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f() {
        assert_eq!(f(), Some(100));
    }

    #[test]
    #[should_panic]
    fn test_pred() {
        pred(0).unwrap();
    }
}

mod a {
    pub mod a_1 {
        pub struct TypeA1 {
            m: usize,
            pub n: usize,
        }

        pub fn run(p: &super::a_1::TypeA1) {
            println!("{}", p.m);
            println!("{}", p.n);
        }
    }

    pub mod a_2 {
        pub fn run(p: &super::a_1::TypeA1) {
            // println!("{}", p.m); アクセスできない
            println!("{}", p.n);
        }
    }
}

mod b {
    mod b_1_outer {
        pub mod b_1_inner {
            pub struct TypeB0;

            // 同じクレート内からのみ見える
            pub struct TypeB1;

            // 親モジュールからのみ見える
            pub struct TypeB2;

            // 親モジュールのみから見える
            pub struct TypeB3;

            // プライベート同義
            pub struct TypeB4;
        }

        fn f() {
            let p1 = b_1_inner::TypeB1;
            let p2 = b_1_inner::TypeB2;
            let p3 = b_1_inner::TypeB3;
            // プライベートなので見えない
            // let p1 = b_1_inner::TypeB4;
        }
    }

    mod b_2 {
        // 再エクスポート
        pub use super::b_1_outer::b_1_inner::TypeB0;
    }

    fn g() {
        let p0 = b_2::TypeB0;
        let p1 = b_1_outer::b_1_inner::TypeB1;
        // 以下、全て見えない
        // let p2 = b_1_outer::b_1_inner::TypeB2;
        // let p3 = b_1_outer::b_1_inner::TypeB3;
        // let p4 = b_1_outer::b_1_inner::TypeB4;
    }
}