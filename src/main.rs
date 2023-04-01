#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
use rand::Rng;
use std::{
    env,
    f32::consts::PI,
    fmt::format,
    fs::File,
    io::Write,
    ops::{Add, Mul, Neg, Sub},
    thread,
};

#[derive(Default, Clone, Copy, Debug)]
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

impl Copy for Sphere {}

impl Vec3 {
    fn dot(self, b: Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    fn cross(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
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
        Vec3 {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        }
    }
    fn multf32(self, b: f32) -> Vec3 {
        Vec3 {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
        }
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

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        self.multf32(rhs)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z)
    }
}

impl Sphere {
    fn intersect(self, r: &Ray) -> f32 {
        let op = self.pos - r.o;
        let eps: f32 = 1e-4;
        // let eps = f32::EPSILON;
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
    // let x = 0.5 + 255.0 * x.clamp(0.0, 0.99999).powf(1.0 / 2.2);
    let x = 0.5 + 255.0 * x.clamp(0.0, 0.99999);
    x as i32
}

fn intersect(spheres: &Vec<Sphere>, r: &Ray) -> (bool, f32, usize) {
    let n = spheres.len();
    // let inf = f32::INFINITY;
    let inf = 1e20;
    let mut t = inf;
    let mut id = 0;
    for i in 0..n {
        let d = spheres[i].intersect(r);
        if !d.eq(&0.0) && d < t {
            t = d;
            id = i;
        }
    }
    (t < inf, t, id)
}

fn radiance(spheres: &Vec<Sphere>, r: &Ray, depth: i32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut t = f32::INFINITY;
    let mut id = 0;
    let res = intersect(spheres, r);
    let t = res.1;
    let id = res.2;
    if !res.0 || depth > 50{
        return Vec3{
            ..Default::default()
        };
    }
    let sphere = spheres[id];
    let x = r.o + r.d*t;
    let n = (x-sphere.pos).norm();
    
    return Vec3{
        ..Default::default()
    };
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
                z: 81.6f32,
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
                y: 1e5,
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
    let width = 1024;
    let hight = 768;
    let size = width * hight;
    let mut color_buffer: Vec<Vec3> = Vec::new();
    color_buffer.resize(
        size,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );
    let cam = Ray {
        o: Vec3 {
            x: 50.0,
            y: 52.0,
            z: 295.6,
        },
        d: Vec3 {
            x: 0.0,
            y: -0.042612,
            z: -1.0,
        }
        .norm(),
    };
    let cx = Vec3 {
        x: (width as f32) * 0.5135 / (hight as f32),
        y: 0.0,
        z: 0.0,
    };
    let cy = cx.cross(cam.d).norm() * 0.5135;
    // let mut r = Vec3 {
    //     ..Default::default()
    // };

    let args = env::args();
    let argc = args.len();
    let args: Vec<String> = args.collect();
    println!("{argc}");
    let mut simple = 1;
    if argc == 2 {
        let num = args[1].as_str();
        simple = num.parse().unwrap();
        simple = simple / 4;
    }
    let mut rng = rand::thread_rng();

    // render
    for y in 0..hight {
        println!("Rendering ({}/{} spp)", y, hight);
        
        for x in 0..width {
            let i = (hight - y - 1) * width + x;
            for sx in 0..2 {
                for sy in 0..2 {
                    let mut r = Vec3 {
                        x:0.0,y:0.0,z:0.0
                    };
                    for s in 0..simple {
                        
                    }
                    color_buffer[i] = color_buffer[i]
                        + Vec3 {
                            x: r.x.clamp(0.0, 1.0),
                            y: r.y.clamp(0.0, 1.0),
                            z: r.z.clamp(0.0, 1.0),
                        } * 0.25;
                }
            }
        }
    }

    // write to ppm
    let mut file = match File::create("image.ppm") {
        Ok(f) => f,
        Err(_) => {
            panic!()
        }
    };
    file.write(format!("P3\n{} {}\n{}\n", width, hight, 255).as_bytes());
    for i in 0..size {
        let color = color_buffer[i];
        if i == 0 {
            println!("{}", toInt(color.x));
            let line = format!(
                "{} {} {} \n",
                toInt(color.x),
                toInt(color.y),
                toInt(color.z)
            );
            println!("{}", line);
        }
        file.write(format!("{} {} {} ", toInt(color.x), toInt(color.y), toInt(color.z)).as_bytes());
    }
}
