//! # 正規表現エンジン用クレート。
//!
//! ## 利用例
//!
//! ```
//! use reg;
//! let expr = "a(bc)+|c(def)*"; // 正規表現
//! let line = "cdefdefdef"; // マッチ対象文字列
//! reg::do_matching(expr, line, true); // 幅優先探索でマッチング
//! reg::print(expr); // 正規表現のASTと命令列を表示
//! ```
mod engine;
mod helper;

pub use engine::{do_matching, print};