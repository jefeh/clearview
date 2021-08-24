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

use crate::Screen;
pub use usize as ScreenId;

pub enum MouseStatus {
    Idle,
    Clicked
}

pub enum KeyboardStatus {
    Idle,
    KeyDown(u8),
    KeyUp(u8),
}

pub struct UpdateData {
    pub mouse_position : (u32, u32),
    pub mouse_status: MouseStatus
}

/// A window contains screens. 
/// This is a trait, so the implementation can use a determined technology.
pub trait Window {
    /// Creates a Window
    /// params:
    ///     * caption: Title of the window
    ///     * width: Width of the window
    ///     * height: Height of the window
    ///     * resizable: Whether the window can be resized
    //fn new (caption: &str, width: u32, height: u32, resizable: bool) -> Self;

    /// Adds a Screen to the window and returns its id.
    /// params:
    ///     * screen:   The screen to be added
    /// returns:
    ///     The id of the screen.
    fn add_screen(&mut self, screen: Screen) -> ScreenId;

    /// Window loop
    fn execute(self);
}