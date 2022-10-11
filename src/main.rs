mod permutation_table;
mod math;

use math::Vec2;
use permutation_table::PermutationTable;

const SEED: u64 = 0x5EED;

fn main() {
    let perm_table = PermutationTable::new(SEED);
    println!("{:?}", perm_table);
}

/// # 10 Minute-crafts tutorial (i think)
/// - Locate the unit cube. Aka x.floor() & 0xff
///     Gives us a point defining the unit square, from this the rest of the cube can be found.
///     We "bind?" the value with a bit-wise and, so that we don't index-OoB our permutation table.
/// 
/// - Find relative x, y in cube (t = t - t.floor()). 
///     This is the relative coords of the point in the unit cube.
/// 
/// - Compute the 4 distance vectors. 
///     use a random gradient vector
/// 
/// - Get an 'influence value' by computing the dot product of distance vector and gradient vector. 
///     this is the bit with the "& 0b11" for randomness, so pick a random gradient vector for each distance vector
/// 
/// - Compute "fade curve" for x, y -> u, v. 
///     The fade can be seen like a weight value for the point.
///     This is to make the noise look better, as it makes changes in the noise more gradual.
/// 
/// - Linear interpolation time! Linearly interpolate between the 'influence values' and the relative input coords. 
///     So, if the influence values are g1..g4 and our relative coords are u and v, then
///     
///     let l1 = lerp(g1, g2, u);
///     let l2 = lerp(g3, g4, u);
/// 
///     let res = lerp(x1, x2, v);
/// 
/// That should be it. NOW WRITE IT >:(
/// 
/// i actually don't know if this is all, i just hope it is
fn perlin2d(mut point: Vec2) -> f64 {



    todo!();
}

/// Interpolate "fade curves" through the fith degree interpotation as described, 
/// by Ken Perlin in 2002 (this is sometimes called a 'Quintic s-curve')
fn fade(t: f64) -> f64 {
    t * t * t * ( t * ( t * 6.0 - 15.0 ) + 10.0 )
}

/// Simple linear interpolation
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

/// Gives us a random gradiant and gives us the dot product of it and the point parsed
fn grad(hash: u8, point: Vec2) -> f64 {

    match hash & 0b11 { // Check the last two bits as a way to randomly choose a gradient
        0 =>  point.x + point.y,    // ( 1,  1) * (x, y)
        1 =>  point.x - point.y,    // ( 1, -1) * (x, y)
        2 => -point.x + point.y,    // (-1,  1) * (x, y)
        3 => -point.x - point.y,    // (-1, -1) * (x, y)
        _ => unreachable!(),
    }
}
