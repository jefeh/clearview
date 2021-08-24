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

use crate::{Direction, Sense};

/// Describes a rectangle in the screen
///     * total:        Width and height of the window
///     * prop_coords:  Coordinates (x, y, x2, y2): top left corner is (1.0, -1.0) and bottom right (-1.0, 1.0)
///     * pixel_coords: prop_coordinates mapped to pixel_space:  top left corner is (0, 0) and bottom right (width, height)
/// pixel_coords is only a mapped version of prop_coords, stored only for efficiency in order not to compute it when needed.
#[derive(Debug)]
pub struct Space {
    total: (u32, u32),
    pub prop_coords: (f32, f32, f32, f32),
    pub pixel_coords: (u32, u32, u32, u32)
}

impl Space {

    /// Creates a Space with the dimensions os the screen
    pub fn new(width: u32, height: u32) -> Space {
        //assert!(width > 0 && height > 0);
        Space { 
            total: (width.max(1_u32), height.max(1_u32)),
            prop_coords: (-1.0f32, 1.0f32, 1.0f32, -1.0f32),
            pixel_coords:  (0, 0, width, height)
        }
    }

    /// Creates a Space passing the dimensions of the screen and the coordinates in the space (-1.0, 1.0, 1.0, -1.0).
    /// This is a private method that computes pixel_coords automatically from prop_coords.
    fn new_helper(screen_width: u32, screen_height: u32, x: f32, y: f32, x2:f32, y2: f32) -> Space {
        assert!(/*screen_width > 0 && screen_height > 0*/ true
            && x >= -1.0f32 && x <= 1.0f32 && x2 >= -1.0 && x2 <= 1.0f32
            && y >= -1.0f32 && y <= 1.0f32 && y2 >= -1.0 && y2 <= 1.0f32
            && x <= x2 && y2 <= y);
        Space { 
            total: (screen_width.max(1_u32), screen_height.max(1_u32)),
            prop_coords: (x, y, x2, y2),
            pixel_coords:  ((((x + 1.0f32) * 0.5f32) * screen_width as f32) as u32, 
                            (((-y + 1.0f32) * 0.5f32) * screen_height as f32) as u32, 
                            (((x2 + 1.0f32) * 0.5f32) * screen_width as f32) as u32, 
                            (((-y2 + 1.0f32) * 0.5f32) * screen_height as f32) as u32)
        }
    }

    pub fn is_inside(&self, x: u32, y: u32) -> bool {
        x > self.pixel_coords.0 && x < self.pixel_coords.2 && y > self.pixel_coords.1 && y < self.pixel_coords.3
    }

    /// Splits a space into two, cutting with a proportion
    pub fn split(self, proportion: f32, direction: Direction, sense: Sense) -> (Space, Space) {
        assert!(proportion > 0.0f32 && proportion < 1.0f32);
        match (direction, sense) {
            (Direction::Horizontal, Sense::Forward) => {
                let width = self.prop_coords.2 - self.prop_coords.0;
                (
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0, self.prop_coords.1, 
                        self.prop_coords.0 + (width * proportion), self.prop_coords.3),
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0 + (width * proportion), self.prop_coords.1, 
                        self.prop_coords.2, self.prop_coords.3)
                )
            },
            (Direction::Horizontal, Sense::Backwards) => {
                let width = self.prop_coords.2 - self.prop_coords.0;
                (
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0 + (width * (1.0f32 - proportion)), self.prop_coords.1, 
                        self.prop_coords.2, self.prop_coords.3),
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0, self.prop_coords.1, 
                        self.prop_coords.0 + (width * (1.0f32 - proportion)), self.prop_coords.3)
                )
            },
            (Direction::Vertical, Sense::Forward) => {
                let height = self.prop_coords.3 - self.prop_coords.1;
                (
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0, self.prop_coords.1, 
                        self.prop_coords.2, self.prop_coords.1 + (height * proportion)),
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0, self.prop_coords.1 + (height * proportion), 
                        self.prop_coords.2, self.prop_coords.3)
                )
            },
            (Direction::Vertical, Sense::Backwards) => {
                let height = self.prop_coords.3 - self.prop_coords.1;
                (
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0, self.prop_coords.1 + (height * (1.0f32 - proportion)), 
                        self.prop_coords.2, self.prop_coords.3),
                    Space::new_helper(self.total.0, self.total.1, 
                        self.prop_coords.0, self.prop_coords.1, 
                        self.prop_coords.2, self.prop_coords.1 + (height * (1.0f32 - proportion)))
                )
            }
        }
    }

    /// Splits this apace into two, the former with a given length in pixels
    pub fn trim(self, pixels: u32, direction: Direction, sense: Sense) -> (Space, Space) {
        assert!(pixels > 0);
        match direction {
            Direction::Horizontal => {
                let width = ((pixels << 1) as f32 / self.total.0 as f32).min(self.prop_coords.2 - self.prop_coords.0);
                match sense {
                    Sense::Forward => (
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.0, self.prop_coords.1, 
                            self.prop_coords.0 + width, self.prop_coords.3),
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.0 + width, self.prop_coords.1, 
                            self.prop_coords.2, self.prop_coords.3)
                        ),
                    Sense::Backwards => (
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.2 - width, self.prop_coords.1, 
                            self.prop_coords.2, self.prop_coords.3),
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.0, self.prop_coords.1, 
                            self.prop_coords.2 - width, self.prop_coords.3)
                        )
                }
            },
            Direction::Vertical => {
                let height = ((pixels << 1) as f32 / self.total.1 as f32).min(self.prop_coords.1 - self.prop_coords.3);
                match sense {
                    Sense::Forward => (
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.0, self.prop_coords.1, 
                            self.prop_coords.2, self.prop_coords.1 - height),
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.0, self.prop_coords.1 - height, 
                            self.prop_coords.2, self.prop_coords.3)
                        ),
                    Sense::Backwards => (
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.0, self.prop_coords.3 + height, 
                            self.prop_coords.2, self.prop_coords.3),
                        Space::new_helper(self.total.0, self.total.1, 
                            self.prop_coords.0, self.prop_coords.1, 
                            self.prop_coords.2, self.prop_coords.3 + height)
                        )
                }
            }
        }
    }

    /// Divides this space into a given number of equally sized spaces
    pub fn divide(self, divs: usize, direction: Direction, sense: Sense) -> Vec<Space> {
        assert!(divs > 0);
        let mut result = Vec::with_capacity(divs);
        match direction {
            Direction::Horizontal => {
                let width = (self.prop_coords.2 - self.prop_coords.0) / divs as f32;
                match sense {
                    Sense::Forward => {
                        for i in 0..divs {
                            let new_x = self.prop_coords.0 + width * i as f32;
                            result.push(Space::new_helper(self.total.0, self.total.1, 
                                new_x, self.prop_coords.1, 
                                new_x + width, self.prop_coords.3));
                        }
                    },
                    Sense::Backwards => {
                        for i in 0..divs {
                            let new_x = self.prop_coords.0 + width * (divs - i) as f32;
                            result.push(Space::new_helper(self.total.0, self.total.1, 
                                new_x, self.prop_coords.1, 
                                new_x + width, self.prop_coords.3));
                        }
                    }
                }
            },
            Direction::Vertical => {
                let height = (self.prop_coords.3 - self.prop_coords.1) / divs as f32;
                match sense {
                    Sense::Forward => {
                        for i in 0..divs {
                            let new_y = self.prop_coords.1 + height * i as f32;
                            result.push(Space::new_helper(self.total.0, self.total.1, 
                                self.prop_coords.0, new_y, 
                                self.prop_coords.2, new_y + height));
                        }
                    },
                    Sense::Backwards => {
                        for i in 0..divs {
                            let new_y = self.prop_coords.1 + height * (divs - i) as f32;
                            result.push(Space::new_helper(self.total.0, self.total.1, 
                                self.prop_coords.0, new_y, 
                                self.prop_coords.2, new_y + height));
                        }
                    }
                }
            }
        }
        result
    }

    pub fn shrink(&self, pixels_hor: u32, pixels_vert: u32) -> Self {
        let horiz_prop = ((pixels_hor << 1) as f32 / self.total.0 as f32).min((self.prop_coords.2 - self.prop_coords.0) * 0.5_f32);
        let vert_prop = ((pixels_vert << 1) as f32 / self.total.1 as f32).min((self.prop_coords.1 - self.prop_coords.3) * 0.5_f32);
        Space::new_helper(self.total.0, self.total.1, 
            self.prop_coords.0 + horiz_prop, 
            self.prop_coords.1 - vert_prop, 
            self.prop_coords.2 - horiz_prop, 
            self.prop_coords.3 + vert_prop)
    }


}