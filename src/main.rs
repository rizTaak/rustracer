#![feature(unboxed_closures)]
#![feature(fn_traits)]

// todo: disable it once done coding!!
#![cfg_attr(debug_assertions, allow(dead_code))]

mod pbrt;

fn main() {
    let vec2 = pbrt::Vector2f::new(1.0, 2.0);
    let vec3 = pbrt::Vector3f::new(3.0, 4.0, 5.0);
    println!("{:?}", vec2);
    println!("{:?}", vec3);
}
