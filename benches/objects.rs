
use bhh_rs::*;

#[derive(Debug, Clone)]
pub struct Mesh {
    points: Vec<Float3>,
}

impl Mesh {
    pub fn new(points: usize, radius: f32) -> Self {
        debug_assert!(radius > 0.);
        debug_assert!(points > 0);

        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut coord = move || rng.gen_range(-radius, radius);

        let points = (0..points)
            .map(|_| loop {
                let point = Float3 {
                    x: coord(),
                    y: coord(),
                    z: coord(),
                };
                if point.len() < radius {
                    break point;
                }
            })
            .collect();

        Self { points }
    }
}

#[derive(Debug, Clone)]
pub struct Object<'a> {
    pub mesh: &'a Mesh,
    pub position: Float3,
}

impl<'a> Object<'a> {
    pub fn get_calculated_aabb(&self) -> AABB {
        let mut result = AABB::default();
        self.calculate_aabb(&mut result);
        result
    }

    pub fn calculate_aabb(&self, out: &mut AABB) {
        let xyz = &self.position + &self.mesh.points[0];
        out.min = xyz.clone();
        out.max = xyz;

        self.mesh.points.iter().for_each(|point| {
            let xyz = &self.position + point;

            out.min = out.min.min(&xyz);
            out.max = out.max.max(&xyz);
        });
    }
}

