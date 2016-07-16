/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut numbers = (2..n).collect::<Vec<u32>>();
    let mut primes = Vec::new();

    while numbers.len() > 0 {
        let prime = numbers.remove(0);
        primes.push(prime);
        for i in 1..(n/prime+1) {
            match numbers.binary_search(&(i * prime)) {
                Ok(position) => {
                    numbers.remove(position);
                },
                _ => ()
            }
        }
    }
    primes
}