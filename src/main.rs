static MAX_LIMIT: i32 = 28123;
static LOWER_LIMIT: i32 = 24;

fn main() {
	let mut sum = 0;
	for i in LOWER_LIMIT..MAX_LIMIT {
		if is_abundant(i) {
			sum += i;
		}
	}
	println!("The sum is: {}", sum);
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

	// Find the divisors.
	for i in start..end {
		if nbr % i == 0 {
			divisors.push(i);
		}
	}

	// Sum it up.
	let sum: i32 = divisors.iter().sum();
	return sum > nbr;
}
