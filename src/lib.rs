#![doc = include_str!("../README.md")]
#![allow(clippy::tabs_in_doc_comments)]

use proc_macro::TokenStream;

mod pipe;
mod tokens;
mod utils;

#[proc_macro]
/// A macro that lets you use the pipeline operator with [Hack-like syntax](https://docs.hhvm.com/hack/expressions-and-operators/pipe).
///
/// ```rs
/// fn add(a: usize, b: usize) -> usize {
/// 	a + b
/// }
///
/// fn orig_and_double(num: usize) -> (usize, usize) {
/// 	(num, num * 2)
/// }
///
/// let num = 4;
///
/// let piped = pipe! {
/// 	num |> add(2, _) |> orig_and_double(_),
/// 	(_, doubled) |> doubled as isize,
/// };
///
/// assert_eq!(piped, 12isize);
/// ```
///
/// You can pass any expression in as the input.
///
/// Notice that you can chain pipelines with `,`s to destructure the result of the previous pipeline.
pub fn pipe(input: TokenStream) -> TokenStream {
	pipe::pipe(input)
}
