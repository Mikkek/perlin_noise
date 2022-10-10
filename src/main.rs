mod permutation_table;
use permutation_table::PermutationTable;

const SEED: u64 = 0x5EED;

fn main() {
    let perm_table = PermutationTable::new(SEED);
    println!("{:?}", perm_table);
}
