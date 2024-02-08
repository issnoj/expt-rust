pub fn exec() {
    impl_display::run();
    list_iterator::run();
    custom_error::run();
    closure_type::run();
}

// クロージャーの型比較
mod closure_type {
    use std::any::{Any, TypeId};

    pub fn run() {
        // クロージャーの型はコンパイル時にユニークな型を割り当てるため false になる
        let f = |x: i32| x * x;
        let g = |x: i32| x * x;
        println!("{}", get_type(&f) == get_type(&g));

        // 以下のようにすれば true になる
        let ff = closure();
        let gg = closure();
        println!("{}", get_type(&ff) == get_type(&gg));
    }

    fn closure() -> impl Fn(i32) -> i32 {
        |x| x * x
    }

    fn get_type<T: Any>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }
}

// 動的ディスパッチを利用した独自エラー型の実装 (Debug, Error, Error を実装)
mod custom_error {
    use std::{error::Error, fmt::Display};

    pub fn run() {
        let numbers = [3, 5, 11];
        for &number in &numbers {
            match double(number) {
                Ok(result) => println!("結果: {}", result),
                Err(e) => println!("エラー: {}", e)
            }
        }
    }

    // エラーA
    #[derive(Debug)]
    struct ErrorA;

    impl Display for ErrorA {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error A")
        }
    }

    impl Error for ErrorA {}

    // エラーB
    #[derive(Debug)]
    struct ErrorB;

    impl Display for ErrorB {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error B")
        }
    }

    impl Error for ErrorB {}

    // 二乗数して 100 を超えたらエラーA、10 を超えたらエラーBを返す
    fn double(x: i32) -> Result<i32, Box<dyn Error>> {
        let result = x.pow(2);
        if result > 100 {
            return Err(Box::new(ErrorA));
        } else if result > 10 {
            return Err(Box::new(ErrorB));
        }
        Ok(result)
    }
}

// リストに対するイテレータの実装と、シリアライズ/デシリアライズ
mod list_iterator {
    use std::iter::Iterator;
    use serde::{Deserialize, Serialize};
    use std::{fs::File, io::prelude::*, path::Path};

    pub fn run() {
        let list = List::new()
            .cons(0).cons(1).cons(2);

        // イテレータの利用
        println!("=== イテレータの利用");
        for x in list.iter() {
            println!("{x}");
        }
        let mut it = list.iter();
        println!("{:?}", it.next().unwrap());
        println!("{:?}", it.next().unwrap());
        println!("{:?}", it.next().unwrap());

        // シリアライズ
        println!("=== シリアライズ");
        let json = serde_json::to_string(&list).unwrap();
        println!("JSON: {} bytes", json.len());
        println!("{json}");
        let yaml = serde_yaml::to_string(&list).unwrap();
        println!("YAML: {} bytes", yaml.len());
        println!("{yaml}");

        // デシリアライズ
        println!("=== デシリアライズ");
        let list = serde_json::from_str::<List<i32>>(&json).unwrap();
        println!("{:?}", list);

        // ファイルへの書き出し
        println!("=== ファイルへの書き出し");
        let path = Path::new("test.yaml");
        let mut f = File::create(path).unwrap();
        f.write_all(yaml.as_bytes()).unwrap();

        // ファイルから読みだし
        println!("=== ファイルから読みだし");
        let mut f = File::open(path).unwrap();
        let mut yaml = String::new();
        f.read_to_string(&mut yaml).unwrap();
        let list = serde_yaml::from_str::<List<i32>>(&yaml).unwrap();
        println!("{:?}", list);
    }

    // リストを表す型
    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum List<T> {
        Node { data: T, next: Box<List<T>> },
        Nil,
    }

    impl<T> List<T> {
        fn new() -> List<T> {
            List::Nil
        }

        // リストを消費して先頭にdataを追加したリストを返す
        fn cons(self, data: T) -> List<T> {
            List::Node {
                data,
                next: Box::new(self),
            }
        }

        fn iter(&self) -> ListIter<T> {
            ListIter { elm: self }
        }
    }

    // 不変イテレータを表す型
    struct ListIter<'a, T> {
        elm: &'a List<T>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.elm {
                List::Node { data, next } => {
                    self.elm = next;
                    Some(data)
                }
                List::Nil => None,
            }
        }
    }
}

// Displayトレイトの実装
mod impl_display {
    pub fn run() {
        let n = ImaginaryNumber { real: 3.0, img: 4.0 };
        println!("{n}")
    }

    use std::fmt::{Display, Formatter};

    struct ImaginaryNumber {
        pub real: f64,
        pub img: f64,
    }

    impl Display for ImaginaryNumber {
        fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
            // write!マクロを用いてフォーマッタに文字列を書き込む
            write!(f, "{} + {}i", self.real, self.img)
        }
    }
}