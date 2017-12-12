pub fn nth(x: u32) -> Result<u64, &'static str> {
    match x {
        0               => Err("Invalid Input."),
        1               => Ok(2),
        x if x < 1_000  => Ok(t_div(x)), // expecting a u64
        _               => Ok(sieve(x)),
    }
}

fn t_div(x: u32) -> u64 {
    let mut primes: Vec<u64> = Vec::with_capacity(x as usize);

    primes.push(2);

    if x > 1 {
        primes.push(3);
    }

    let mut next_checked = *primes.last().unwrap() + 2;

    while primes.len() < x as usize {
        if primes.iter().all(|&i| next_checked % i != 0) {
            primes.push(next_checked)
        }
        next_checked += 2;
    }
    *primes.last().unwrap()
}

fn sieve(x:u32) -> u64 {
    let x = x as f64;

    let upper_limit = (x *((x*x.ln()).ln())).floor() as usize;
    let mut prime_indices = vec![true; upper_limit];

    for i in 2..upper_limit {
        if prime_indices[i] {
            let mut counter = 0;
            while (i.pow(2) + (counter * i)) < upper_limit  {
                prime_indices[(i.pow(2) + (counter * i)) as usize] = false;
                counter += 1;
            }
        }
    }

    let mut prime_x = 1;
    let mut prime_count = 0;
    for i in 2..upper_limit {
        if prime_indices[i] {
            prime_count += 1;
            prime_x = i
        }
        if prime_count == x as u64 {
            break
        }
    }
    prime_x as u64
}
