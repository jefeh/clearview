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

/// Styles for the widgets
///     * Classic:  Hard edge widgets, straight lines
///     * Modern:   Rounded widgets
#[derive(Clone, Copy)]
pub enum WidgetStyle {
    Classic,
    Modern,
}

/// Colors for the widgets
#[derive(Clone, Copy)]
pub struct Theme {
    pub style: WidgetStyle,
    pub active: Color,
    pub inactive: Color,
    pub right: Color,
    pub wrong: Color,
    pub hover: Color,
    pub border: Color,
    pub background: Color,
}

impl Theme {
    pub fn new() -> Self {
        Theme { 
            style: WidgetStyle::Modern,
            active: Color {r: 0.75294_f32, g: 0.75294_f32, b: 0.75294_f32, a: 1_f32}, 
            inactive: Color {r: 1_f32, g: 1_f32, b: 1_f32, a: 1_f32},
            right: Color {r: 0_f32, g: 1_f32, b: 0_f32, a: 1_f32},
            wrong: Color {r: 1_f32, g: 0_f32, b: 0_f32, a: 1_f32},
            hover: Color {r: 0_f32, g: 1_f32, b: 1_f32, a: 1_f32},
            border: Color {r: 0_f32, g: 0_f32, b: 0_f32, a: 1_f32},
            background: Color {r: 0.9412_f32, g: 0.9412_f32, b: 0.9412_f32, a: 1_f32},
        }
    }
}
