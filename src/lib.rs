extern crate rand;

use rand::prelude::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
  thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
  exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
  exponentiation(b_pub, a, p)
}

pub fn exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
  // Using binary exponentiation
  if modulus == 1 {
    return 0;
  }
  let new_base = base % modulus;
  if exponent > 0 {
    compute_exponent(new_base, exponent, modulus, 1)
  } else {
    1
  }
}

pub fn compute_exponent(base: u64, exponent: u64, modulus: u64, result: u64) -> u64 {
  let new_result = if exponent % 2 == 1 {
    (result * base) % modulus
  } else {
    result
  };
  let shifted_exponent = exponent >> 1;
  let new_base = (base * base) % modulus;
  if shifted_exponent > 0 {
    compute_exponent(new_base, shifted_exponent, modulus, new_result)
  } else {
    new_result
  }
}
