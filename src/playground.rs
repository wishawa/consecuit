struct Obj<'a> {
	inside: &'a mut i32
}

fn take_and_return<'a>(s: Obj<'a>) -> Obj<'a> {
	s
}

fn run_tr_generic<F>(func: F)
where F: Fn(Obj<'_>) -> Obj<'_> {
	let mut v = 42;
	let s: Obj<'_> = Obj {
		inside: &mut v
	};
	func(s);
}

fn run_me() {
	let mut v = 42;
	let s: Obj<'_> = Obj {
		inside: &mut v
	};
	take_and_return(s);
}
/*
fn run_me_2<'a>() {
	let mut v = 42;
	let s: Obj<'a> = Obj {
		inside: &mut v
	};
	take_and_return(s);
}
*/
static TEST_BORROW: &() = &();

fn test<'a>(a: &'a ()) -> &'static () {
	TEST_BORROW
}