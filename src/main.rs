mod permutation_table;
mod math;

use image::{ImageBuffer, Luma};
use math::Vec2;
use permutation_table::PermutationTable;

const SEED: u64 = 0x5EED;
const SCALE: f64 = 100.0;

fn main() {
    let perm_table = PermutationTable::new(SEED);
    let mut image = ImageBuffer::new(1024, 1024);

    for i in 0..1024 {
        for j in 0..1024 {
            let mut point = Vec2::new(
                (i as f64) / SCALE,
                (j as f64) / SCALE,
            );
            let mut val = perlin2d(&mut point, &perm_table);

            val = (val + 1.0) / 2.0;    // Normalize val

            image.put_pixel(i, j, Luma([ (val * 255.0) as u8 ]))
        }
    }
    let filename = "perlin_test";
    let path = format!("images/{}.png", filename);

    let res = image.save(path);

    match res {
        Ok(_) => println!("Image saved"),
        Err(e) => println!("Error saving image\n{}", e),
    }
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
/// - Compute "fade curve" for x, y -> u, v. 
///     The fade can be seen like a weight value for the point.
///     This is to make the noise look better, as it makes changes in the noise more gradual.
/// 
/// - Compute the 4 distance vectors. 
///     use a random gradient vector
/// 
/// - Get an 'influence value' by computing the dot product of distance vector and gradient vector. 
///     this is the bit with the "& 0b11" for randomness, so pick a random gradient vector for each distance vector
/// 
/// - Linear interpolation time! Linearly interpolate between the 'influence values' and the relative input coords. 
///     Remember to do the linear interpolation in the correct order, if you don't you will end up wondering why your results are not what you would expect. 
///     This can easily lead to you spending A LOT of time trying to figure out why everything looks ok, but nothing is, until you realize, that 
///     the problem was actually just the result of your own stupidity. This will undoubtedly lead to more self hatred, as you realize how mutch time you have wasted 
///     due to your own idiotic mistake. Just as an arbitraty example.
fn perlin2d(point: &mut Vec2, perm_table: &PermutationTable) -> f64 {

// Find unit square:
    let unit_x = (point.x.floor() as usize) & 0xff;
    let unit_y = (point.y.floor() as usize) & 0xff;

// Relative x, y:
    point.x -= point.x.floor();
    point.y -= point.y.floor();

// Fade curves:
    let u = fade(point.x);  // The "weight" value for x, y
    let v = fade(point.y);

// Compute distance vectors:
    let d00 = Vec2::new(    // (x, y) - (0, 0)
        point.x - 0.0,
        point.y - 0.0,
    );
    let d01 = Vec2::new(    // (x, y) - (0, 1)
        point.x - 0.0,
        point.y - 1.0,
    );
    let d10 = Vec2::new(    // (x, y) - (1, 0)
        point.x - 1.0,
        point.y - 0.0,
    );
    let d11 = Vec2::new(    // (x, y) - (1, 1)
        point.x - 1.0,
        point.y - 1.0,
    );

// Compute dot product between distance vectors and a gradient vector:
    let g00 = grad( perm_table.hash(&[unit_x + 0, unit_y + 0]), d00 );  // gradient( hash(0, 0), d00 )
    let g01 = grad( perm_table.hash(&[unit_x + 0, unit_y + 1]), d01 );  // gradient( hash(0, 1), d01 )
    let g10 = grad( perm_table.hash(&[unit_x + 1, unit_y + 0]), d10 );  // gradient( hash(1, 0), d10 )
    let g11 = grad( perm_table.hash(&[unit_x + 1, unit_y + 1]), d11 );  // gradient( hash(1, 1), d11 )


// Linear interpolation:
    let l1 = lerp(u, g00, g10); // This is realy important to not fuck up (i did lol)
    let l2 = lerp(u, g01, g11);

    let res = lerp(v, l1, l2);

// Final value:
    res
}

/// Interpolate "fade curves" through the fith degree interpotation as described, 
/// by Ken Perlin in 2002 (this is sometimes called a 'Quintic s-curve').
fn fade(t: f64) -> f64 {
    t * t * t * ( t * ( t * 6.0 - 15.0 ) + 10.0 )
}

/// Simple linear interpolation.
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

/// Picks a random gradiant vector and gives us the dot product of it and the point parsed.
/// 
/// The four gradient vectors for our square are (1, 1), (1, -1), (-1, 1) and (-1, -1). 
/// We simply pick one at random and return the dot product between the gradient vector and the vector provided.
fn grad(hash: u8, point: Vec2) -> f64 {

    match hash & 0b11 { // Check the last two bits as a way to randomly choose a gradient
        0 =>  point.x + point.y,    // ( 1,  1) * (x, y)
        1 =>  point.x - point.y,    // ( 1, -1) * (x, y)
        2 => -point.x + point.y,    // (-1,  1) * (x, y)
        3 => -point.x - point.y,    // (-1, -1) * (x, y)
        _ => unreachable!(),
    }
}
