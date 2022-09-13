extern crate core;

use std::*;
use std::borrow::Borrow;
use std::cell::Cell;
use std::thread::sleep;
use grid::Grid;
use grid::*;
use rand::prelude::*;
use crate::functions::{neighborhood, printScreen, world_gen};
use crate::graphics::{cunt, griddle_print, SunPrint};
use kiss3d::*;
use kiss3d::nalgebra::{Vector3, UnitQuaternion, point, Translation2};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::{PlanarSceneNode, SceneNode};

mod graphics;
mod functions;

//Notes
// Rand Chance doesn't work as intended
const WorldSize: usize = 7;
const Perm: usize = 100;
const SPEED: u64 = 1;
const CellSize:f32 = 50.0;

const RandChance: i32 = 1;
const X_Cell: usize = 0;
const Y_Cell: usize = 0;


fn main(){

    game_of_life();

}

fn game_of_life() {

    //Initialize the world
    let mut window = Window::new_with_size("Rust GOL", 1920, 1020);
    let mut world1: Grid<u8> = world_gen(WorldSize, RandChance);
    window.set_background_color(0.0, 0.0, 0.3);

    //let mut world1:Grid<u8> = grid! [[0,0,0]
    //                                [1,1,1]
    //                                [0,0,0]];



    for x in 0..Perm {
        {
            let mut world2: Grid<u8> = Grid::new(WorldSize, WorldSize);
            for y in 0..WorldSize {
                for x in 0..WorldSize {
                    let mut neighbors = neighborhood(&world1, x, y);
                    //println!("{:?}", neighbors);
                    // for alive Cells
                    //println!("{:?}",world1.get(y,x));
                    if world1.get(y, x) == Some(&1) {
                        match neighbors {
                            0 => { *world2.get_mut(y, x).unwrap() = 0 }
                            1 => { *world2.get_mut(y, x).unwrap() = 0 }
                            2 => { *world2.get_mut(y, x).unwrap() = 1 }
                            3 => { *world2.get_mut(y, x).unwrap() = 1 }
                            4 => { *world2.get_mut(y, x).unwrap() = 0 }
                            5 => { *world2.get_mut(y, x).unwrap() = 0 }
                            6 => { *world2.get_mut(y, x).unwrap() = 0 }
                            7 => { *world2.get_mut(y, x).unwrap() = 0 }
                            8 => { *world2.get_mut(y, x).unwrap() = 0 }
                            val => {}
                        }
                    }
                    // For dead cells
                    if world1.get(y, x) == Some(&0) {
                        match neighbors {
                            0 => { *world2.get_mut(y, x).unwrap() = 0 }
                            1 => { *world2.get_mut(y, x).unwrap() = 0 }
                            2 => { *world2.get_mut(y, x).unwrap() = 0 }
                            3 => { *world2.get_mut(y, x).unwrap() = 1 }
                            4 => { *world2.get_mut(y, x).unwrap() = 0 }
                            5 => { *world2.get_mut(y, x).unwrap() = 0 }
                            6 => { *world2.get_mut(y, x).unwrap() = 0 }
                            7 => { *world2.get_mut(y, x).unwrap() = 0 }
                            8 => { *world2.get_mut(y, x).unwrap() = 0 }
                            val => {}
                        }
                    }




                    for x in 0..WorldSize {
                        for y in 0..WorldSize {

                            let yy = y as f32;
                            let xx = x as f32;
                            match world1.get(y, x) {
                                Some(1) => {
                                    let mut Jack = window.add_rectangle(CellSize, CellSize);
                                    Jack.append_translation((&Translation2::new(xx * (CellSize + 1.0), yy * (CellSize + 1.0))));
                                    Jack.set_color(1.0, 1.0, 1.0);


                                }
                                Some(0) => {
                                    let mut Jack = window.add_rectangle(CellSize, CellSize);
                                    Jack.append_translation((&Translation2::new(xx * (CellSize + 1.0), yy * (CellSize + 1.0))));
                                    Jack.set_color(0.0, 0.0, 0.0);
                                    }
                                Some(val) => {}
                                None => {}


                            }

                        }

                    }


                }
            }
            world1 = printScreen(world2);
            println!();
            window.render();
        }
    }

}






