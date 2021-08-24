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

use crate::Color;

pub type ImageId = usize;

pub trait Surface {
    fn load_image(&mut self, location: &str) -> ImageId;
    fn clear(&mut self, color: &Color);
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, line_color: &Color);
    fn draw_rectangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, border_color: Option<&Color>, fill_color: Option<&Color>);
    fn draw_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, border_color: Option<&Color>, fill_color: Option<&Color>);
    fn draw_ellipse(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, border_color: Option<&Color>, fill_color: Option<&Color>);
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, border_color: Option<&Color>, fill_color: Option<&Color>);
    fn draw_rounded_rectangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, rad_pixel: u32, border_color: Option<&Color>, fill_color: Option<&Color>);
    fn draw_text(&mut self, text: &str, x1: f32, y1: f32, x2: f32, y2: f32, text_color: &Color);
    fn draw_image(&mut self, img: ImageId, x1: f32, y1: f32, x2: f32, y2: f32);
}