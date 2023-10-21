use indenter::{indented, Indented};
use std::fmt::{Display, Formatter, Result};

/// Print source code in a block.
///
/// This function accepts a writing function, and returns a [`Block`]. When the `Block`
/// is printed via the `fmt::Display` trait, it writes an open brace and new line,
/// indents the formatter, and calls the function to write the contents of the block.
/// Then, it unindents the formatter and writes a close brace.
///
/// This allows for generating source code without having to manually insert
/// opening/closing braces for blocks or worry about indentation.
///
/// # Examples
///
/// ```ignore
/// writeln!(
///     f,
///     "if (ok) {if_true} else {if_false}"
///     if_true = block(|mut f| {
///         writeln!(f, "console.log(\"ok\");")
///     }),
///     if_false = block(|mut f| {
///         writeln!(f, "console.log(\"err\");")
///     })
/// )?;
/// ```
/// This writes:
/// ```js
/// if (ok) {
///   console.log("ok");
/// } else {
///   console.log("err");
/// }
/// ```
pub fn block<F>(f: F) -> Block<F>
where
    F: Fn(Indented<Formatter>) -> Result,
{
    Block(f)
}

/// Generate source code in an indented block.
///
/// See [`block`] for more info.
pub struct Block<F>(F)
where
    F: Fn(Indented<Formatter>) -> Result;

impl<F> Display for Block<F>
where
    F: Fn(Indented<Formatter>) -> Result,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{{")?;
        self.0(indented(f).with_str("  "))?;
        write!(f, "}}")
    }
}

/// Write arbitrarily complex statements inline.
///
/// When generating source code, sometimes it's difficult to see where parantheses
/// balance or remember to add a semicolon at the end of a long expression. This
/// function allows you to wrap the display logic of a complex expression into
/// a single type, making it easier to format the code around the expression.
///
/// # Examples
///
/// Variable assignment without forgetting the semicolon at the end.
/// ```ignore
/// writeln!(f, "const out = {};", expr(|mut f| {
///     write!(f, "1 + 2")
/// }))?;
/// ```
/// This writes:
/// ```js
/// const out = 1 + 2;
/// ```
pub fn expr<F>(f: F) -> Expr<F>
where
    F: Fn(&mut Formatter) -> Result,
{
    Expr(f)
}

/// Generate source code as an expression.
///
/// See [`expr`] for more info.
pub struct Expr<F>(F)
where
    F: Fn(&mut Formatter) -> Result;

impl<F> Display for Expr<F>
where
    F: Fn(&mut Formatter) -> Result,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0(f)
    }
}

/// Write an immediately invoked function expression (IIFE).
///
/// This function accepts a closure that writes the contents of the IIFE,
/// and generates the proper wrapping and indentation.
///
/// # Examples
///
/// ```ignore
/// writeln!(f, "const out = {};", iife(|mut f| {
///     writeln!(f, "const out = {{}};")?;
///     writeln!(f, "out.a = 7;")?;
///     writeln!(f, "return out;")?;
/// }))?;
/// ```
/// This generates
/// ```js
/// const out = (() => {
///   const out = {};
///   out.a = 7;
///   return out;
/// })();
/// ```
pub fn iife<F>(f: F) -> IIFE<F>
where
    F: Fn(Indented<Formatter>) -> Result,
{
    IIFE(f)
}

/// An `fmt::Display` type returned by [`iife`].
#[allow(clippy::upper_case_acronyms)]
pub struct IIFE<F>(F)
where
    F: Fn(Indented<Formatter>) -> Result;

impl<F> Display for IIFE<F>
where
    F: Fn(Indented<Formatter>) -> Result,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(() => {})()", block(&self.0))
    }
}

pub fn ts_doc<F>(f: F) -> TsDoc<F>
where
    F: Fn(Indented<Formatter>) -> Result,
{
    TsDoc(f)
}

/// An `fmt::Display` type returned by [`ts_doc`].
pub struct TsDoc<F>(F)
where
    F: Fn(Indented<Formatter>) -> Result;

impl<F> Display for TsDoc<F>
where
    F: Fn(Indented<Formatter>) -> Result,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "/**")?;
        self.0(indented(f).with_str(" * "))?;
        writeln!(f, " */")
    }
}
