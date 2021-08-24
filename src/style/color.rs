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

/// Definition of a color
///     * r: red [0.0 - 1.0]
///     * g: green [0.0 - 1.0]
///     * b: blue [0.0 - 1.0]
///     * a: alpha transparency [0.0 - 1.0]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}


impl Color {
    pub fn approach(&mut self, color: Color) {
        let prop = 0.025_f32;
        let (adv_r, adv_g, adv_b, adv_a) = (
            (self.r - color.r) * prop, 
            (self.g - color.g) * prop, 
            (self.b - color.b) * prop, 
            (self.a - color.a) * prop);
        self.r -= adv_r;
        self.g -= adv_g;
        self.b -= adv_b;
        self.a -= adv_a;
    }
}