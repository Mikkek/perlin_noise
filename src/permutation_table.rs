use std::fmt::Debug;

use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng, seq::SliceRandom,
};
use rand_xoshiro::SplitMix64;

const TABLE_SIZE: usize = 256;

/// A pseudo-random permutation of the numbers [0; 256[
pub struct PermutationTable {
    // Maybe just use a Vec<u8>?
    values: [u8; TABLE_SIZE],
}

impl PermutationTable {

    /// Generates a new, random, PermutationTable from the given seed.
    pub fn new(seed: u64) -> Self {
        let mut prng = SplitMix64::seed_from_u64(seed);
        prng.gen()
    }

    /// Hash into the PermutationTable with an arbitrary amount of points.
    /// 
    /// This is a simple Pearson Hash.
    /// 
    /// # Panics
    /// 
    /// If i made a very big mistake
    pub fn hash(&self, input: &[isize]) -> u8 {
        let index = input
            .iter()
            .map(|&a| (a & 0xff) as usize)  // Reduce values to the last 8 bits
            .reduce(|a, b| self.values[a ^ b] as usize)
            .unwrap();
        self.values[index]
    }
}

/// Define how a standard distribution should produce a PermutationTable.
/// 
/// This means that anything implementing the [Rng] trait can produce a randomized PermutationTable, 
/// but since a PermutationTable mut be created though the new() method, we only use [SplitMix64].
/// 
/// ## Note
/// It might be an idea to come back to this and find a better implementation for randomizing the PermutationTable. 
/// As it is right now "bad" PermutationTables could be created giving repetetive results, but it's probably not a big deal.
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
