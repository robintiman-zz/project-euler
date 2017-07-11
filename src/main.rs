static MAX_LIMIT: i32 = 28123;
fn main() {
	is_abundant(12);
}

// Check if the given nbr is abundant.
fn is_abundant(nbr: i32) -> bool {
	let end;
	let start = 1;
	let mut divisors = Vec::new();
	if nbr % 2 == 0 {
		end = nbr / 2 + 1;		
	} else {
		end = nbr / 3 + 1;
	}

	for i in start..end {
		if nbr % i == 0 {
			println!("{}", i);
			divisors.push(i);
		}
	}
	return false;
}
