use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock, Mutex},
    thread::sleep,
    time::Duration,
};

// 銀行口座から二人の人が並行で引き出す
pub fn bank() {
    let x = Arc::new(Mutex::new(100_000));
    let x1 = x.clone(); // 参照カウンタをインクリメント
    let x2 = x.clone(); // 参照カウンタをインクリメント

    // A さんの引き出し
    let h1 = std::thread::spawn(move || {
        let mut guard = x1.lock().unwrap();
        *guard -= 20_000;
    });

    // B さんの引き出し
    let h2 = std::thread::spawn(move || {
        let mut guard = x2.lock().unwrap();
        *guard -= 30_000;
    });
    h1.join().unwrap();
    h2.join().unwrap();

    // 最終的な金額を表示
    let final_amount = *x.lock().unwrap();
    println!("最終的な銀行口座の金額: {}", final_amount);
}

// 美術館の展示内容の入れ替え
pub fn gallery() {
    let mut gallery = BTreeMap::new();
    gallery.insert("葛飾北斎", "あ");
    gallery.insert("ミュシャ", "い");

    // RwLock と Arc を利用して共有可能に
    let gallery = Arc::new(RwLock::new(gallery));

    // join ハンドラ
    let mut hdls = Vec::new();

    // ３人の客
    for n in 0..3 {
        // 客を表すスレッドを生成
        let gallery = gallery.clone(); // 参照カウンタをインクリメント
        let hdl = std::thread::spawn(move || {
            // 美術館の展示を８周見て回る
            for _ in 0..8 {
                {
                    // 読み込みロック
                    let guard = gallery.read().unwrap();
                    if n == 0 {
                        // １周目のみ美術館の展示内容を表示
                        for (key, value) in guard.iter() {
                            print!("{n} {key}:{value}, ");
                        }
                        println!();
                    }
                }
                sleep(Duration::from_secs(1));
            }
        });
        hdls.push(hdl);
    }

    // 美術館スタッフ
    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            println!("{n} 展示内容の入れ替え");
            // 書き込みロック
            let mut guard = gallery.write().unwrap();
            // 展示内容を入れ替え
            if n % 2 == 0 {
                guard.clear();
                guard.insert("ゴッホ", "星月夜");
                guard.insert("エッシャー", "滝");
            } else {
                guard.clear();
                guard.insert("葛飾北斎", "あ");
                guard.insert("ミュシャ", "い");
            }
            sleep(Duration::from_secs(2));
        }
    });

    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join().unwrap();
}

