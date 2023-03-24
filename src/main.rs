use std::ops::{Add, Sub};

#[derive(Default, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Ray {
    o: Vec3,
    d: Vec3,
}

#[derive(Clone, Copy)]
enum Refl_t {
    DIFF,
    SPEC,
    REFR,
}

#[derive(Clone)]
struct Sphere {
    radius: f32,
    pos: Vec3,
    color: Vec3,
    emission: Vec3,
    refl: Refl_t,
}

impl Copy for Sphere {

}

impl Vec3 {
    fn dot(self, b: Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    fn cross(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: self.x * b.z - self.z * b.x,
            z: self.x * b.y - self.y * b.x,
        }
    }
    fn sub(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
    fn add(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
    fn norm(self) -> Vec3 {
        let length = f32::sqrt(self.clone().dot(self.clone()));
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
    fn mult(self, b: Vec3) -> Vec3 {
        Vec3 { x: self.x*b.x, y: self.y*b.y, z: self.z*b.z }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sphere {
    fn intersect(self, r: &Ray) -> f32 {
        let op = self.pos - r.o;
        let eps: f32 = f32::EPSILON;
        let b: f32 = op.dot(r.d);
        let mut dt: f32 = b * b - op.dot(op) + self.radius * self.radius;
        if dt < 0.0 {
            return 0.0;
        } else {
            dt = dt.sqrt();
        }
        if b - dt > eps {
            return b - dt;
        } else {
            if b + dt > eps {
                return b + dt;
            } else {
                return 0.0;
            }
        }
    }
}

fn toInt(x: f32) -> i32 {
    let x = x.clamp(0.0, 0.99999).powf(1.0 / 2.2);
    x as i32
}

fn intersect(spheres: &Vec<Sphere>, r: &Ray) -> (bool,f32,usize) {
    let n = spheres.len();
    let inf = f32::INFINITY;
    let mut t = inf;
    let mut id = 0usize;
    for i in 0..n {
        let d = spheres[i].intersect(r);
        if (d-0.0).abs() >= f32::EPSILON && d < t {
            t = d;
            id = i;
        }
    }
    (t<inf,t,id)
}

fn radiance(r:&Ray, depth: i32) -> Vec3{
    // todo 
    Vec3{x:0.0,y:0.0,z:0.0}
}
fn main() {
    println!("Hello, world!");
    let spheres = vec![
        Sphere {
            radius: 1e5,
            pos: Vec3 {
                x: 1e5f32 + 1f32,
                y: 40.8,
                z: 81.6,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0.75,
                y: 0.25,
                z: 0.25,
            },
            refl: Refl_t::DIFF,
        },
        Sphere {
            radius: 1e5,
            pos: Vec3 {
                x: -1e5f32 + 99f32,
                y: 40.8f32,
                z: 81.3f32,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0.25,
                y: 0.25,
                z: 0.75,
            },
            refl: Refl_t::DIFF,
        },
        Sphere {
            radius: 1e5,
            pos: Vec3 {
                x: 50f32,
                y: 40.8f32,
                z: 1e5f32,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
            refl: Refl_t::DIFF,
        },
        Sphere {
            radius: 1e5,
            pos: Vec3 {
                x: 50.0,
                y: 40.8,
                z: -1e5 + 170f32,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            refl: Refl_t::DIFF,
        },
        Sphere {
            radius: 1e5,
            pos: Vec3 {
                x: 50.0,
                y: 40.8,
                z: -1e5 + 170f32,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
            refl: Refl_t::DIFF,
        },
        Sphere {
            radius: 1e5,
            pos: Vec3 {
                x: 50.0,
                y: -1e5 + 81.5,
                z: 81.6,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
            refl: Refl_t::DIFF,
        },
        Sphere {
            radius: 16.5,
            pos: Vec3 {
                x: 27.0,
                y: 16.5,
                z: 47.0,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0.999,
                y: 0.999,
                z: 0.999,
            },
            refl: Refl_t::SPEC,
        },
        Sphere {
            radius: 16.5,
            pos: Vec3 {
                x: 73.0,
                y: 16.5,
                z: 78.0,
            },
            emission: Vec3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            color: Vec3 {
                x: 0.999,
                y: 0.999,
                z: 0.999,
            },
            refl: Refl_t::REFR,
        },
        Sphere {
            radius: 600.0,
            pos: Vec3 {
                x: 50.0,
                y: 681.6 - 0.27,
                z: 81.6,
            },
            emission: Vec3 {
                x: 12.0,
                y: 12.0,
                z: 12.0,
            },
            color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            refl: Refl_t::DIFF,
        },
    ];
    
}
