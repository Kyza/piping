# piping

`piping` provides a `pipe!` macro that allows you to use the pipeline operator in Rust.

```rs
let wrapped = orig_and_double(add(2, num)).1 as isize;

let piped = pipe! {
	num |> add(2, __) |> orig_and_double(__),
	(_, doubled) |> doubled as isize,
};
```

## Features

- [x] [Hack-like syntax.](https://docs.hhvm.com/hack/expressions-and-operators/pipe)
- [x] Multiple pipelines in one statement.
- [x] Destructuring of previous pipeline results.

## Docs

Documentation is provided on [docs.rs](https://docs.rs/piping).

## How does it work?

```rs
fn add(a: usize, b: usize) -> usize {
	a + b
}

fn orig_and_double(num: usize) -> (usize, usize) {
	(num, num * 2)
}

let num = 4;

let piped = pipe! {
	num |> add(2, __) |> orig_and_double(__),
	(_, doubled) |> doubled as isize,
};

// Expands to...
let piped = {
	let __ = num;
	let __ = add(2, __);
	let (_, doubled) = orig_and_double(__);
	doubled as isize
};
```

You can pass any expression in as the input.

Notice that you can chain pipelines with `,`s to destructure the result of the previous pipeline.

The macro also tries to optimize the generated code to minimize the amount of reassigning done.
