use count::*;

#[test]
fn it_decrements() {

	let count = Count::new(1);
	
	let has_next = match count.next() {
		None => false,
		Some(_) => true
	};
	
	assert!(has_next);

}

#[test]
fn it_stops() {

	let count = Count::new(0);
	
	let has_no_next = match count.next() {
		None => true,
		Some(_) => false
	};
	
	assert!(has_no_next);

}
