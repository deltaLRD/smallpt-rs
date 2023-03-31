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
    let x = 0.5 + 255.0 * x.clamp(0.0, 0.99999).powf(1.0 / 2.2);
    x as i32
}

fn intersect(spheres: &Vec<Sphere>, r: &Ray) -> (bool, f32, usize) {
    let n = spheres.len();
    let inf = f32::INFINITY;
    let mut t = inf;
    let mut id = 0usize;
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
    // todo
    let mut rng = rand::thread_rng();
    let mut depth = depth;
    let mut t: f32 = 0.0;
    let mut id = 0;
    let res = intersect(spheres, r);
    t = res.1;
    id = res.2;
    if !res.0 {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let obj = spheres[id];
    let x = r.o + r.d.multf32(t);
    let n = (x - obj.pos).norm();
    let nl = if n.dot(r.d) < 0.0 { n } else { -n };
    let mut f = obj.color;
    let p: f32 = if f.x > f.y && f.x > f.z {
        f.x
    } else {
        if f.y > f.z {
            f.y
        } else {
            f.z
        }
    };
    depth += 1;
    if (depth > 5) {
        if rng.gen::<f32>() < p {
            f = f * (1.0 / p);
        } else {
            return obj.emission;
        }
    }
    match obj.refl {
        Refl_t::DIFF => {
            let r1 = 2.0 * PI * rng.gen::<f32>();
            let r2 = rng.gen::<f32>();
            let r2s = r2.sqrt();
            let w = nl;
            let u = if w.x.abs() > 0.1 {
                return Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }
                .cross(w)
                .norm();
            } else {
                return Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }
                .cross(w)
                .norm();
            };
            let v = w.cross(u);
            let d = (u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1.0 - r2).sqrt()).norm();
            return obj.emission + f.mult(radiance(spheres, &Ray { o: x, d: d }, depth));
        }
        Refl_t::SPEC => {
            return obj.emission
                + f.mult(radiance(
                    spheres,
                    &Ray {
                        o: x,
                        d: r.d - n * 2.0 * n.dot(r.d),
                    },
                    depth,
                ));
        }
        Refl_t::REFR => {
            let reflRay = Ray {
                o: x,
                d: r.d - n * 2.0 * n.dot(r.d),
            };
            let into = n.dot(nl) > 0.0;
            let nc = 1.0;
            let nt = 1.5;
            let nnt = if into { nc / nt } else { nt / nc };
            let ddn = r.d.dot(nl);
            let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);
            if cos2t < 0.0 {
                return obj.emission + f.mult(radiance(spheres, &reflRay, depth));
            }
            let tdir = (r.d * nnt
                - n * ((if into { 1.0 } else { -1.0 }) * (ddn * nnt + cos2t.sqrt())))
            .norm();
            let a = nt - nc;
            let b = nt + nc;
            let R0 = a * a / (b * b);
            let c = 1.0 - (if into { -ddn } else { tdir.dot(n) });
            let Re = R0 + (1.0 - R0) * c.powi(5);
            let Tr = 1.0 - Re;
            let P = 0.25 + Re * 0.5;
            let RP = Re / P;
            let TP = Tr / (1.0 - P);
            return obj.emission
                + f.mult(if depth > 2 {
                    if rng.gen::<f32>() < P {
                        radiance(spheres, &reflRay, depth) * RP
                    } else {
                        radiance(spheres, &Ray { o: x, d: tdir }, depth) * TP
                    }
                } else {
                    radiance(spheres, &reflRay, depth) * Re
                        + radiance(spheres, &Ray { o: x, d: tdir }, depth) * Tr
                });
        }
    }
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
    let width = 1024;
    let hight = 768;
    let size = width * hight;
    let mut color_buffer: Vec<Vec3> = Vec::new();
    color_buffer.resize(
        size,
        Vec3 {
            x: 0.5,
            y: 0.5,
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
    let mut r = Vec3 {
        ..Default::default()
    };

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

    // render test
    assert_eq!(
        Vec3 {
            x: 3.0,
            y: 3.0,
            z: 9.0
        },
        radiance(
            &spheres,
            &Ray {
                o: Vec3 {
                    x: 87.208499,
                    y: 67.047286,
                    z: 161.476524
                },
                d: Vec3 {
                    x: 0.265775,
                    y: 0.107481,
                    z: -0.958025
                }
            },
            0
        )
    );

    // render
    for y in 0..hight {
        println!("\rRendering ({}/{} spp)", y, hight);
        for x in 0..width {
            let i = (hight - y - 1) * width + x;
            for sx in 0..2 {
                for sy in 0..2 {
                    r = Vec3 {
                        ..Default::default()
                    };
                    for s in 0..simple {
                        let r1 = 2.0 * rng.gen::<f32>();
                        let dx = if r1 < 1.0 {
                            r1.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r1).sqrt()
                        };
                        let r2 = 2.0 * rng.gen::<f32>();
                        let dy = if r2 < 1.0 {
                            r2.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r2).sqrt()
                        };
                        let d =
                            cx * (((sx as f32 + 0.5 + dy) / 2.0 + x as f32) / width as f32 - 0.5);
                        let depth = 0;
                        r = r + radiance(
                            &spheres,
                            &Ray {
                                o: cam.o + d * 140.0,
                                d: d.norm(),
                            },
                            depth,
                        ) * (1.0 / simple as f32);
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
