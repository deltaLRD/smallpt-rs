
#[derive(Default, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Ray {
    o: Vec3,
    d: Vec3
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
        Vec3 { x: self.x-b.x, y: self.y-b.y, z: self.z-b.z }
    }
    fn add(self, b: Vec3) -> Vec3 {
        Vec3 { x: self.x+b.x, y: self.y+b.y, z: self.z+b.z }
    }
    fn norm(self) -> Vec3 {
        let length = f32::sqrt(self.clone().dot(self.clone()));
        Vec3 { x: self.x/length, y: self.y/length, z: self.z/length }
    }
}

fn main() {
    println!("Hello, world!");
}
