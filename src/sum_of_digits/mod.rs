


fn add_digits(mut num:i32) -> i32 {
	let mut sum:i32 =0;
	let mut rem;
	while num > 0{
		rem = num%10;
		sum += rem;
		num=num/10;
	}
	return sum;
}

fn sum_of_digits(x: i32) {
	match x{
		ref y if *y < 10 => println!("{:?}", y),
		ref y if *y >= 10 => sum_of_digits(add_digits(*y)),
		_ => println!("{:?}", "not valid")
	}
}

pub fn run() {
	sum_of_digits(123);
}