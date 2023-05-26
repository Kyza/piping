use piping::pipe;

fn add(a: usize, b: usize) -> usize {
	a + b
}

fn orig_and_double(num: usize) -> (usize, usize) {
	(num, num * 2)
}

#[test]
fn multiline() {
	let num = 4;

	let wrapped = orig_and_double(add(2, num)).1 as isize;

	let piped = pipe! {
		num |> add(2, __) |> orig_and_double(__),
		(_, doubled) |> doubled as isize,
	};

	assert_eq!(piped, wrapped);
}

#[test]
fn if_statement() {
	let num = 4;

	let wrapped = if num == 4 { 1 } else { 0 };

	let piped = pipe! {
		num |> if __ == 4 { 1 } else { 0 }
	};

	assert_eq!(piped, wrapped);
}

#[test]
fn normal_underscores() {
	pipe! {
		4 |> {
			let _: Vec<_> = vec![5];
			__
		}
	};
}
