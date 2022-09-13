


extern crate kiss3d;


use grid::Grid;
use kiss3d::*;
use kiss3d::nalgebra::{Vector3, UnitQuaternion, point, Translation2};
use kiss3d::window::Window;
use kiss3d::light::Light;

pub fn cunt() {

}





extern crate nalgebra as na;

use na::{UnitComplex};
/*
pub fn prim() {
    let mut window = Window::new("Kiss3d: rectangle");
    let mut rect = window.add_rectangle(50.0, 150.0);
    let mut circ = window.add_circle(50.0);
    circ.append_translation(&Translation2::new(100.0, 0.0));

    rect.set_color(0.0, 1.0, 0.0);
    circ.set_color(0.0, 0.0, 1.0);

    let rot_rect = UnitComplex::new(0.014);
    let rot_circ = UnitComplex::new(-0.014);

    while window.render() {
        rect.prepend_to_local_rotation(&rot_rect);
    }
}
*/





pub fn griddle_print(){
    let mut window = Window::new("Kiss3d: rectangle");
        for x in 0..10{
            let y = x as f32;
            window.add_rectangle(50.0, 50.0)
                .append_translation((&Translation2::new(y*51.0,0.0)));
        }
    while window.render() {
    }
}

pub fn SunPrint(world: Grid<u8>, Width: u32, Height: u32, WorldSize: usize) -> Grid<u8>{
    let mut window = Window::new_with_size("Rust GOL", Width, Height);
    for y in 0..WorldSize{
        for x in 0..WorldSize{
            world.get(y,x);
        }
    }
    return world;
}