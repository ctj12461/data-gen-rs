use std::io::{Read, Write};

use data_gen::config::Config;
use data_gen::runtime::{Executor, GeneratorEnvironment, SolverEnvironment};

use rand::Rng;

fn main() {
    let mut executor = Executor::new("output/queuing");

    for i in 1..=3 {
        executor.testcase(Config::new(i).insert("max_n", 100).insert("max_m", 100));
    }
    
    for i in 4..=10 {
        executor.testcase(Config::new(i).insert("max_n", 2000).insert("max_m", 2000));
    }

    executor.run(generator, solver, 24);
}

fn generator(mut env: GeneratorEnvironment, config: &Config) {
    let mut rng = rand::thread_rng();

    let max_n = config.get("max_n").unwrap_integer();
    let max_m = config.get("max_m").unwrap_integer();

    let n = rng.gen_range(max_n / 5 * 4..=max_n);
    let m = rng.gen_range(max_m / 5 * 4..=max_m);

    env.input.write_all(format!("{n} {m}\n").as_bytes()).unwrap();
}

fn solver(mut env: SolverEnvironment, _config: &Config) {
    let mut buffer = String::new();
    env.input.read_to_string(&mut buffer).unwrap();
    let mut it = buffer.split_whitespace();
    let n: u64 = it.next().unwrap().trim().parse().unwrap();
    let m: u64 = it.next().unwrap().trim().parse().unwrap();
    env.output
        .write_all(format!("{}\n", solve(n, m, 998244353)).as_bytes())
        .unwrap();
}

fn solve(n: u64, m: u64, p: u64) -> u64 {
    let sum1 = perm(n, n, p) * perm(n + 1, 2, p) % p * perm(n + 3, m, p) % p;
    let sum2 = perm(n, n, p) * (2 * (n + 1)) % p * comb(n + 2, m - 1, p) % p * perm(m, m, p) % p;
    (sum1 + sum2) % p
}

fn perm(n: u64, m: u64, p: u64) -> u64 {
    if n < m {
        0
    } else {
        (n - m + 1..=n).fold(1u64, |acc, x| acc * x % p)
    }
}

fn comb(n: u64, m: u64, p: u64) -> u64 {
    if n < m {
        0
    } else {
        perm(n, m, p) * power(perm(m, m, p), p - 2, p) % p
    }
}

fn power(mut x: u64, mut y: u64, p: u64) -> u64 {
    let mut res = 1;

    while y != 0 {
        if y % 2 == 1 {
            res = res * x % p;
        }

        x = x * x % p;
        y /= 2;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOD: u64 = 998244353;

    #[test]
    fn testcase() {
        assert_eq!(12, solve(1, 1, MOD));
        assert_eq!(37440, solve(3, 4, MOD));
        assert_eq!(221760, solve(5, 2, MOD));
        assert_eq!(0, solve(1, 10, MOD));
        assert_eq!(448039733, solve(1998, 2000, MOD));
        assert_eq!(579403511, solve(1579, 1452, MOD));
    }
}
