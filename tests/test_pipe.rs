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
		num |> add(2, _) |> orig_and_double(_),
		(_, doubled) |> doubled as isize,
	};

	assert_eq!(piped, wrapped);
}

#[test]
fn if_statement() {
	let num = 4;

	let wrapped = if num == 4 { 1 } else { 0 };

	let piped = pipe! {
		num |> if _ == 4 { 1 } else { 0 }
	};

	assert_eq!(piped, wrapped);
}
