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

/// Used for space and layout operations
///     * Horizontal: Makes a row
///     * Vertical: Makes a column
#[derive(Debug, Copy, Clone)]
pub enum Direction {
	Horizontal,
	Vertical
}

/// Used for space and layout operations
///     * Forward: The operation is performed from left to right or from top to bottom
///     * Backwards: The operation is performed from right to left or from bottom to top 
#[derive(Debug, Copy, Clone)]
pub enum Sense {
	Forward,
	Backwards
}

