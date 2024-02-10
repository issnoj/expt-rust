//! 正規表現
pub mod engine;
mod helper;

use helper::DynError;
use std::{env, fs::File, io::{BufRead, BufReader}};

/// 指定ファイルから正規表現にマッチする行を表示する
///
/// # 引数
///
/// 第一引数には正規表現、第二引数にはファイル名を指定する
pub fn run() -> Result<(), DynError> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }
    Ok(())
}

/// ファイルをオープンし、行ごとにマッチングを行う。
///
/// マッチングはそれぞれの行頭から1文字ずつずらして行い、
/// いずれかにマッチした場合に、その行がマッチしたものとみなす。
///
/// たとえば、abcdという文字列があった場合、以下の順にマッチが行われ、
/// このいずれかにマッチした場合、与えられた正規表現にマッチする行と判定する。
///
/// - abcd
/// - bcd
/// - cd
/// - d
fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    engine::print(expr).expect("TODO: panic message");
    println!();

    for line in reader.lines() {
        let line = line?;
        // i は UTF-8 の文字単位でのインデックス
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], true)? {
                println!("{line}");
                break;
            }
        }
    }
    Ok(())
}

// 単体テスト
#[cfg(test)]
mod tests {
    use crate::ch6::engine::do_matching;
    use crate::ch6::helper::{safe_add, SafeAdd};

    #[test]
    fn test_safe_add() {
        let n: usize = 10;
        assert_eq!(Some(30), n.safe_add(&20));

        let n: usize = !0;
        assert_eq!(None, n.safe_add(&1));

        let mut n: usize = 10;
        assert!(safe_add(&mut n, &20, || ()).is_ok());

        let mut n: usize = !0;
        assert!(safe_add(&mut n, &1, || ()).is_err());
    }

    #[test]
    fn test_matching() {
        // パースエラー
        assert!(do_matching("+b", "bbb", true).is_err());
        assert!(do_matching("*b", "bbb", true).is_err());
        assert!(do_matching("|b", "bbb", true).is_err());
        assert!(do_matching("?b", "bbb", true).is_err());

        // パース成功、マッチ成功
        assert!(do_matching("abc|def", "def", true).unwrap());
        assert!(do_matching("(abc)*", "abcabc", true).unwrap());
        assert!(do_matching("(ab|cd)+", "abcdcd", true).unwrap());
        assert!(do_matching("abc?", "ab", true).unwrap());
        assert!(do_matching("abc?d", "abcd", true).unwrap());
        assert!(do_matching("abc?d", "abd", true).unwrap());
        assert!(do_matching("ab*", "a", true).unwrap());
        assert!(do_matching("ab*", "aabb", true).unwrap());
        assert!(do_matching("((ab+)+)+", "ababababab", true).unwrap());
        assert!(do_matching("((((a*)*)*)*)", "aaaaaaaaa", true).unwrap());
        assert!(do_matching("(a*)*b", "aaaaaaaaab", true).unwrap());
        assert!(do_matching("(a*)*b", "b", true).unwrap());
        assert!(do_matching("a**b", "aaaaaaaaab", true).unwrap());
        assert!(do_matching("a**b", "b", true).unwrap());

        // パース成功、マッチ失敗
        assert!(!do_matching("abc|def", "efa", true).unwrap());
        assert!(!do_matching("(ab|cd)+", "", true).unwrap());
        assert!(!do_matching("abc?", "acb", true).unwrap());
    }
}