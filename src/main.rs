use std::fmt::Debug;

use rand::{rngs::SmallRng, SeedableRng, prelude::Distribution,
    distributions::Standard, seq::SliceRandom, Rng};
use rand_xoshiro::SplitMix64;

const SEED: u64 = 0x5EED;
const TABLE_SIZE: usize = 256;

fn main() {
    let perm_table = PermutationTable::new(SEED);
    println!("{:?}", perm_table);
}

struct PermutationTable {
    // Maybe just use a Vec<u8>?
    values: [u8; TABLE_SIZE],
}

impl PermutationTable {
    fn new(seed: u64) -> Self {
        let mut prng = SplitMix64::seed_from_u64(seed);
        
        prng.gen()
    }
}

/// Define how a standart distribution should produce a PermutationTable
impl Distribution<PermutationTable> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PermutationTable {
        // The sorted table
        let mut seq: Vec<u8> = (0..TABLE_SIZE)
            .into_iter()
            .map(|val| val as u8)
            .collect();
        
        // Shuffle. Now it's a permutation
        seq.shuffle(rng);

        let mut perm_table = PermutationTable { values: [0; TABLE_SIZE] };

        // Insert the values into a PermutationTable and return it
        seq.into_iter()
            .zip(perm_table.values.iter_mut())
            .for_each( |(seq_val, perm_val)| {
                *perm_val = seq_val;
            });

        perm_table
    }
}

/// A pretier output than had we simply used a derive macro
impl Debug for PermutationTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::from("PermutationTable {\n");
        for i in 0..TABLE_SIZE/16 {
            text.push_str("\t");
            for j in 0..16 {
                let mut val = self.values[ i * 16 + j ].to_string();
                while val.len() < 3 {
                    val = format!(" {}", val);
                }
                text.push_str( &format!( "{}, ", val ) );
            }
            text.push_str("\n");
        }
        text.push_str("}");
        
        write!(f, "{}", text)
    }
}
