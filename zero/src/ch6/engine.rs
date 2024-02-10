//! 正規表現エンジン
mod codegen;
mod evaluator;
mod parser;

use std::fmt::{self, Display};
use crate::ch6::helper::DynError;

/// 正規表現と文字列をマッチング
///
/// # 利用例
///
/// ```
/// use zero::ch6::engine;
/// engine::do_matching("abc|(de|cd)+", "decddede", true);
/// ```
///
/// # 引数
///
/// expr に正規表現、line にマッチ対象とする文字列を与える
/// is_depth が true の場合は深さ優先探索、false の場合は幅優先探索を利用
///
/// # 返り値
///
/// エラーなく実行でき、かつマッチングに **成功** した場合は Ok(true) を返し、
/// エラーなく実行でき、かつマッチングに **失敗** した場合は Ok(false) を返す
///
/// 入力された正規表現にエラーがあったり、内部的な実装エラーがある場合は Err を返す
pub fn do_matching(expr: &str, line: &str, is_depth: bool)
                   -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();

    Ok(evaluator::eval(&code, &line, is_depth)?)
}

/// 正規表現をパースしてコード生成し、
/// ASTと命令列を標準出力に表示。
///
/// # 利用例
///
/// ```
/// use zero::ch6::engine;
/// engine::print("abc|(de|cd)+");
/// ```
///
/// # 返り値
///
/// 入力された正規表現にエラーがあったり、内部的な実装エラーがある場合はErrを返す。
pub fn print(expr: &str) -> Result<(), DynError> {
    println!("EXPR: {expr}");
    let ast = parser::parse(expr)?;
    println!("AST : {:?}", ast);
    let code = codegen::get_code(&ast)?;
    print!("CODE: ");
    for (i, instruction) in code.iter().enumerate() {
        if i != 0 {
            print!("      ");
        }
        println!("{:04} {}", i, instruction);
    }
    Ok(())
}

/// アセンブリ命令の表現と対応する型
#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) =>
                write!(f, "split {:>04}, {:>04}", addr1, addr2),
        }
    }
}
