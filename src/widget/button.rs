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

use crate::{Surface, Space, Color};

pub fn paint_button(space: &Space, surface: &mut dyn Surface, color_fg: &Color, color_bg: &Color, color_border: &Color) {
    let (x1, y1, x2, y2) = space.prop_coords;
    surface.draw_rectangle(x1, y1, x2, y2, Some(&color_border), Some(&color_bg))
}