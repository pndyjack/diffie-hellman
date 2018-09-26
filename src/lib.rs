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

pub fn exponentiation(b: u64, e: u64, modulus: u64) -> u64 {
  // Using binary exponentiation
  if modulus == 1 {
    return 0;
  }
  let mut result = 1;
  let mut base = b % modulus;
  let mut exponent = e;
  while exponent > 0 {
    if exponent % 2 == 1 {
      result = (result * base) % modulus;
    }
    exponent = exponent >> 1;
    base = (base * base) % modulus;
  }
  result
}
