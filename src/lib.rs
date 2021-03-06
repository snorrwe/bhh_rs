//! # Bounding Half-Space Hierarchy
//!
//!
#![feature(slice_partition_at_index)]

#[cfg(test)]
extern crate rand;
extern crate rayon;

use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Clone, Default)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for &Float3 {
    type Output = Float3;

    fn add(self, rhs: Self) -> Float3 {
        Float3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for Float3 {
    type Output = Float3;

    fn add(self, rhs: Self) -> Float3 {
        &self + &rhs
    }
}

impl Float3 {
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn len(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn min(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    pub fn max(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AABB {
    pub min: Float3,
    pub max: Float3,
}

impl AABB {
    pub fn new(min: Float3, max: Float3) -> Self {
        debug_assert!(min.x <= max.x);
        debug_assert!(min.y <= max.y);
        debug_assert!(min.z <= max.z);

        Self { min, max }
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}

/// Compares two `AABB`s by the given direction
pub fn bhh_compare(dir: u8, a: &AABB, b: &AABB) -> Ordering {
    match dir {
        0 => a.min.x.partial_cmp(&b.min.x).unwrap_or(Ordering::Equal),
        1 => a.min.y.partial_cmp(&b.min.y).unwrap_or(Ordering::Equal),
        2 => a.min.z.partial_cmp(&b.min.z).unwrap_or(Ordering::Equal),
        3 => (-(a.max.x + a.max.y + a.max.z))
            .partial_cmp(&-(b.max.x + b.max.y + b.max.z))
            .unwrap_or(Ordering::Equal),
        _ => unimplemented!(),
    }
}

pub fn bhh_reject(dir: u8, aabb: &AABB, query: &AABB) -> bool {
    match dir {
        0 => query.max.x < aabb.min.x,
        1 => query.max.y < aabb.min.y,
        2 => query.max.z < aabb.min.z,
        3 => -(query.min.x + query.min.y + query.min.z) < -(aabb.max.x + aabb.max.y + aabb.max.z),
        _ => unimplemented!(),
    }
}

/// Sort a range of `AABB`s as a `BHH`
pub fn bhh_sort(items: &mut [AABB]) {
    bhh_sort_impl(items, 0)
}

fn bhh_sort_impl(items: &mut [AABB], dir: u8) {
    if items.len() < 2 {
        return;
    }
    let median = items.len() / 2;
    items.partition_at_index_by(median, |a, b| bhh_compare(dir, a, b));
    let (lo, hi) = items.split_at_mut(median);
    let hi = &mut hi[1..]; // skip the median
    rayon::join(
        || bhh_sort_impl(lo, (dir + 1) & 3),
        || bhh_sort_impl(hi, (dir + 1) & 3),
    );
}

/// Return the number of objects intersecting with `query`
/// TODO: return reference to the objects
pub fn bhh_search(items: &[AABB], query: &AABB) -> u32 {
    bhh_search_impl(items, query, 0)
}

fn bhh_search_impl(items: &[AABB], query: &AABB, dir: u8) -> u32 {
    match items.len() {
        0 => 0,
        1 => items[0].intersects(query) as u32,
        _ => {
            let median = items.len() / 2;
            let intersections = bhh_search_impl(&items[..median], query, (dir + 1) & 3);
            if bhh_reject(dir, &items[median], query) {
                return intersections;
            }
            let inter = items[median].intersects(query) as u32;
            intersections + inter + bhh_search_impl(&items[median + 1..], query, (dir + 1) & 3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_search(items: &[AABB], query: &AABB) -> u32 {
        items.iter().filter(|a| a.intersects(query)).count() as u32
    }

    #[test]
    fn test_search_correctness() {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut coord = move || rng.gen_range(-50., 50.);

        const COUNT: usize = 100;

        let unordered = (0..COUNT)
            .map(|_| {
                let min = Float3 {
                    x: coord(),
                    y: coord(),
                    z: coord(),
                };
                let max = Float3 {
                    x: coord(),
                    y: coord(),
                    z: coord(),
                };

                let (min, max) = (min.min(&max), min.max(&max));
                let position = Float3 {
                    x: coord(),
                    y: coord(),
                    z: coord(),
                };
                let min = &min + &position;
                let max = &max + &position;
                AABB::new(min, max)
            })
            .collect::<Vec<_>>();

        let mut ordered = unordered.clone();
        bhh_sort(&mut ordered);

        let query = &unordered[0];

        let naive_unordered = naive_search(unordered.as_slice(), &query);
        let naive_ordered = naive_search(ordered.as_slice(), &query);
        let bhh_result = bhh_search(ordered.as_slice(), &query);

        assert_eq!(naive_ordered, naive_unordered);
        assert_eq!(naive_ordered, bhh_result);
    }
}

