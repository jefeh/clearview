/*
Copyright 2021 Gonzalo Fernández Hernández

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#[macro_use]
extern crate glium;

mod layout;
mod widget;
mod style;
mod messaging;

mod gl;

use layout::*;
use widget::*;
use style::*;
use messaging::*;
use gl::create_window;


mod test;

fn main() {
	/*let mut layout = Layout::new();
    let (child1, child2) = layout.split(0.5f32, Direction::Vertical, Sense::Forward).unwrap();
    let other = child1.split(0.3, Direction::Horizontal, Sense::Forward).unwrap();
    let last_ones = other.1.divide(5, Direction::Horizontal, Sense::Forward).unwrap();
    last_ones[1].split(0.99f32, Direction::Vertical, Sense::Forward);
    println!("{:?}", &layout );*/
    /*let space = Space::new(800u32, 600u32);
    println!("{:?}", &space.split(0.33f32, Direction::Vertical, Sense::Forward) );
    */
    test::test_app::main();
}
