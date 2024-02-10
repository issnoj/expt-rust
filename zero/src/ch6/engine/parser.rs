//! 正規表現の式をパースし、抽象構文木に変換
use std::{error::Error, fmt::{self, Display}, mem::take};

/// 抽象木を表現するための型
#[derive(Debug)]
pub enum AST {
    // a, J などの文字のパターン
    Char(char),
    // +
    Plus(Box<AST>),
    // *
    Star(Box<AST>),
    // ?
    Question(Box<AST>),
    // |
    Or(Box<AST>, Box<AST>),
    // 正規表現の列を表す
    Seq(Vec<AST>),
}

/// パースエラーを表すための型
#[derive(Debug)]
pub enum ParseError {
    // 誤ったエスケープシーケンス
    InvalidEscape(usize, char),
    // 開きカッコなし
    InvalidRightParen(usize),
    // + | * ? の前に式がない
    NoPrev(usize),
    // 閉じカッコなし
    NoRightParen,
    // 空のパターン
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = `{c}`")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NoRightParen => {
                write!(f, "ParseError: no right parenthesis")
            }
            ParseError::Empty => {
                write!(f, "ParseError: empty expression")
            }
        }
    }
}

impl Error for ParseError {}

/// 特殊文字のエスケープ
fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | '|' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

/// + * ? を表す型
enum PSQ {
    Plus,
    Star,
    Question,
}

/// + * ? を AST に変換
///
/// 倒置記法で + * ? の前にパターンがない場合はエラー
fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}

/// | で結合された式を AST に変換
///
/// 例) a | bc | def は AST::Or("a", AST::Or("bc", "def")) となる
fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        seq_or.pop()
    }
}

/// 正規表現を抽象構文木に変換
pub fn parse(expr: &str) -> Result<AST, ParseError> {
    // 内部状態を表す型
    enum ParseState {
        // 文字列処理中
        Char,
        // エスケープシーケンス処理中
        Escape,
    }

    // 現在の Seq のコンテキスト
    let mut seq = Vec::new();
    // 現在の Or のコンテキスト
    let mut seq_or = Vec::new();
    // コンテキストのスタック
    let mut stack = Vec::new();
    // 現在の状態
    let mut state = ParseState::Char;

    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => {
                match c {
                    '+' => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                    '*' => parse_plus_star_question(&mut seq, PSQ::Star, i)?,
                    '?' => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
                    '(' => {
                        // 現在のコンテキストをスタックに保存し、現在のコンテキストを空にする
                        let prev = take(&mut seq);
                        let prev_or = take(&mut seq_or);
                        stack.push((prev, prev_or));
                    }
                    ')' => {
                        // 現在のコンテキストをスタックからポップ
                        if let Some((mut prev, prev_or)) = stack.pop() {
                            // "()" のように式が空の場合はプッシュしない
                            if !seq.is_empty() {
                                seq_or.push(AST::Seq(seq));
                            }

                            // Or を生成
                            if let Some(ast) = fold_or(seq_or) {
                                prev.push(ast);
                            }

                            // 以前のコンテキストを現在のコンテキストにする
                            seq = prev;
                            seq_or = prev_or;
                        } else {
                            // "abc)" のように開きカッコがないのに閉じカッコがある場合はエラー
                            return Err(ParseError::InvalidRightParen(i));
                        }
                    }
                    '|' => {
                        if seq.is_empty() {
                            // "||" "(|abc)" など、式が空の場合はエラー
                            return Err(ParseError::NoPrev(i));
                        } else {
                            let prev = take(&mut seq);
                            seq_or.push(AST::Seq(prev));
                        }
                    }
                    '\\' => state = ParseState::Escape,
                    _ => seq.push(AST::Char(c)),
                }
            }
            ParseState::Escape => {
                // エスケープシーケンス処理
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }

    // 閉じカッコが足りない場合はエラー
    if !stack.is_empty() {
        return Err(ParseError::NoRightParen);
    }

    // "()" のように式が空の場合はプッシュしない
    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }

    // Or を生成し、成功した場合はそれを返す
    if let Some(ast) = fold_or(seq_or) {
        Ok(ast)
    } else {
        Err(ParseError::Empty)
    }
}
