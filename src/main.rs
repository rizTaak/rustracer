#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod pbrt;

fn main() {
    let vec2 = pbrt::Vector2f::new(1.0, 2.0);
    let vec3 = pbrt::Vector3f::new(3.0, 4.0, 5.0);
    println!("{:?}", vec2);
    println!("{:?}", vec3);
}
