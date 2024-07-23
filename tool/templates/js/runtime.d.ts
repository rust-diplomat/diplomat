/// A [char] is a Unicode code point, such as `a`, or `💡`.
/// 
/// The recommended way to obtain a `char` is to create it from a 
/// `String`, which is conceptually a list of `char`s. For example,
/// `'a'.codePointAt(0)` is equal to the `char` `a`.
/// 
/// JS does not have a character/char literal, so integer literals
/// need to be used. For example the Unicode code point U+1F4A1, `💡`,
/// can be represented by `0x1F4A1`. Note that only values in the ranges
/// `0x0..0xD7FF` and `0xE000..0x10FFFF` (both inclusive) are Unicode
/// code points, and hence valid `char`s.
///
/// A `String` can be constructed from a `char` using `String.fromCodePoint()`. 
export type char = number;
export type pointer = number;