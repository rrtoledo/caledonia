use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use rand::RngCore;
use std::cmp::min;

// Helper functions
fn mod_non_power_of_2(hash: &[u8], n: usize) -> usize {
    let epsilon_fail: usize = 1 << 40; // roughly 1 in 10 billion
    let k = log_base2(n * epsilon_fail);
    let k_prime = 1 << k;
    let d = k_prime / n;

    let i = mod_power_of_2(hash, k_prime);

    if i >= d * n {
        panic!("failed: i = {}, d = {}, n = {}, k = {}", i, d, n, k);
    } else {
        i % n
    }
}

fn mod_power_of_2(hash: &[u8], n: usize) -> usize {
    let r = from_bytes_le(hash);
    (n - 1) & r
}

fn log_base2(x: usize) -> usize {
    usize::BITS as usize - x.leading_zeros() as usize - 1
}

fn from_bytes_le(bytes: &[u8]) -> usize {
    let mut array = [0u8; 8];
    let bytes = &bytes[..min(8, bytes.len())];
    array[..bytes.len()].copy_from_slice(bytes);
    usize::from_le_bytes(array)
}

/// Return a 32-byte hash of the given data
pub fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake2bVar::new(32).expect("Failed to construct hasher!");
    hasher.update(data);
    let mut buf = [0u8; 32];
    hasher
        .finalize_variable(&mut buf)
        .expect("Failed to finalize hashing");
    buf
}

/// Return 32-byte hash of the given list of data
pub fn combine_hashes(hash_list: Vec<Vec<u8>>) -> [u8; 32] {
    let mut hasher = Blake2bVar::new(32).expect("Failed to construct hasher!");
    for data in hash_list.iter() {
        hasher.update(data);
    }
    let mut buf = [0u8; 32];
    hasher
        .finalize_variable(&mut buf)
        .expect("Failed to finalize hashing");
    buf
}

pub fn oracle(hash: &[u8], n: usize) -> usize {
    if n.is_power_of_two() {
        mod_power_of_2(hash, n)
    } else {
        mod_non_power_of_2(hash, n)
    }
}

/// Generate a set of items for given set size
/// Items are generated by hashing the current index
pub fn gen_items(size: usize) -> Vec<[u8; 32]> {
    let mut s_p = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    let seed = rng.next_u32();
    for b in 0..size {
        let mut data = Vec::new();
        data.push(seed.to_ne_bytes().to_vec());
        data.push(b.to_ne_bytes().to_vec());
        let item = combine_hashes(data);
        s_p.push(item);
    }
    s_p
}

pub fn format_time(nanos: u128) -> String {
    let mut time = nanos;
    let bounds = [1000, 1000, 1000, 60, 60, 60];
    let units = ["ns", "μs", "ms", "s", "min", "h"];
    for (&bound, &unit) in bounds.iter().zip(units.iter()) {
        if time < bound {
            return time.to_string() + unit;
        }
        time = time / bound;
    }
    (time * 60).to_string() + "h"
}

pub fn format_nb(x: usize) -> String {
    let mut y = x;
    let mut s = String::new();
    let mut b = true;
    while y / 1000 != 0 {
        s = if b {
            (y % 1000).to_string()
        } else {
            (y % 1000).to_string() + &("_".to_string() + &s)
        };
        b = false;
        y = y / 1000;
    }
    if b {
        y.to_string()
    } else {
        (y % 1000).to_string() + "_" + &s
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_oracle() {
        // Test distribution of oracle
        assert!(true);
    }
}
