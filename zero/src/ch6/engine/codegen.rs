//! AST からコード生成を行う
use super::{parser::AST, Instruction};
use crate::ch6::helper::safe_add;
use std::{error::Error, fmt::{self, Display}};

/// コード生成を行う関数
pub fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}

/// コード生成エラーを表す型
#[derive(Debug)]
pub enum CodeGenError {
    PCOverFlow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for CodeGenError {}

/// コード生成器
#[derive(Default, Debug)]
pub struct Generator {
    pc: usize,
    pub insts: Vec<Instruction>,
}

impl Generator {
    /// コード生成を行う関数の入口
    pub fn gen_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        self.gen_expr(ast)?;
        self.inc_pc()?;
        self.insts.push(Instruction::Match);
        Ok(())
    }

    /// プログラムカウンタをインクリメント
    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        // 第二引数にはオーバーフローが起きた場合のエラーを返すクロージャを渡す
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverFlow)
    }

    /// AST をパターン分けし、コード生成を行う関数
    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Or(ast1, ast2) => self.gen_or(ast1, ast2)?,
            AST::Plus(ast) => self.gen_plus(ast)?,
            // AST::Star(ast) => self.gen_star(ast)?,
            // だとたとえば (a*)* の場合に
            //   [Split(1, 5), Split(2, 4), Char('a'), Jump(1), Jump(0), Match]
            // となり評価機で無限ループになる
            // 以下のようにすると
            //   [Split(1, 3), Char('a'), Jump(0), Match]
            // となり無限ループにならない
            AST::Star(e1) => {
                match &**e1 {
                    // `(a*)*`のように`Star`が二重となっている場合にスタックオーバーフローする問題を回避するため、
                    // このような`(((r*)*)*...*)*`を再帰的に処理して1つの`r*`へと変換する。
                    AST::Star(_) => self.gen_expr(&e1)?,
                    AST::Seq(e2) if e2.len() == 1 => {
                        if let Some(e3 @ AST::Star(_)) = e2.get(0) {
                            self.gen_expr(e3)?
                        } else {
                            self.gen_star(e1)?
                        }
                    }
                    e => self.gen_star(&e)?,
                }
            }
            AST::Question(ast) => self.gen_question(ast)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }
        Ok(())
    }

    /// char 命令の生成関数
    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    /// OR 演算子のコード生成器
    ///
    /// ```text
    ///     split L1, L2
    /// L1: e1 のコード
    ///     jmp L3
    /// L2: e2 のコード
    /// L3:
    /// ```
    fn gen_or(&mut self, ast1: &AST, ast2: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L1: e1 のコード
        self.gen_expr(ast1)?;

        // jmp L3
        let jmp_addr = self.pc;
        self.insts.push(Instruction::Jump(0));

        // L2 の値を設定
        self.inc_pc()?;
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        // L2: e2 のコード
        self.gen_expr(ast2)?;

        // L3 の値を設定
        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jmp_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    /// ? 限量子のコード生成器
    ///
    /// ```text
    ///     split L1, L2
    /// L1: e のコード
    /// L2:
    /// ```
    fn gen_plus(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        // e1 のコード
        let l1 = self.pc;
        self.gen_expr(ast)?;

        // split L1, L2
        self.inc_pc()?;
        let split = Instruction::Split(l1, self.pc);
        self.insts.push(split);

        Ok(())
    }

    /// * 限量子のコード生成器
    ///
    /// ```text
    /// L1: split L2, L3
    /// L2: e のコード
    ///     jmp L1
    /// L3:
    /// ```
    fn gen_star(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let l1 = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L2: e1 のコード
        self.gen_expr(ast)?;

        // jmp L1
        self.inc_pc()?;
        self.insts.push(Instruction::Jump(l1));

        // L3 の値を設定
        if let Some(Instruction::Split(_, l3)) = self.insts.get_mut(l1) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailStar);
        }

        Ok(())
    }

    /// ? 限量子のコード生成器
    ///
    /// ```text
    ///     split L1, L2
    /// L1: e1 のコード
    /// L2:
    /// ```
    fn gen_question(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // L1: e1 のコード
        self.gen_expr(ast)?;

        // L2 の値を設定
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    /// 連続する正規表現のコード生成器
    fn gen_seq(&mut self, asts: &[AST]) -> Result<(), CodeGenError> {
        for e in asts {
            self.gen_expr(e)?;
        }
        Ok(())
    }
}