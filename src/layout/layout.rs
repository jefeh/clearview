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
use crate::Widget;
use crate::Space;
use crate::Panel;

/// Describes an operation to divide the space. 
/// This is an intermediate node in the layout tree.
/// 	* Nop: Empty placeholder. Only in this case this can be a leaf.
/// 	* Split: Divides a space into two spaces, with a proportion. (0.5f32 would result in spaces with the same size).
/// 	* Divide: Divides an space into n spaces of the same size.
/// 	* Trim: Divides an space into two, the former with a specific length in pixels.
#[derive(Debug)]
enum LayoutOperation {
	Nop,
	Split{proportion: f32, first: Box<Layout>, second: Box<Layout>},
	Divide(Vec<Layout>),
	Trim{pixels: u32, first: Box<Layout>, second: Box<Layout>},
}

/// A node of the tree that creates the layout of a screen
/// 	* Operation: This is a layout node, it divides the space
/// 	* Group: Layout node, but acts as a colored panel that contains all the children widgets
/// 	* Control: Contains a widget (this node is a leaf)
#[derive(Debug)]
pub enum Layout {
	Operation {op: LayoutOperation, dir: Direction, sense: Sense},
	Group {span: u32, panel: Panel, content: Box<Layout>},
	Control(Widget)
}

impl Default for Layout {
	fn default() -> Self {
		Layout::Operation{op: LayoutOperation::Nop, dir: Direction::Horizontal, sense: Sense::Forward}
	}
}

impl Layout {
	
	/// Generates a no operation node
	pub fn new() -> Self {
		Layout::default()
	}

	/// Split this node into two nodes, proportionaly
	pub fn split(&mut self, proportion: f32, direction: Direction, sense: Sense) -> Result<(&mut Layout, &mut Layout), &str> {
        assert!(proportion > 0.0f32 && proportion < 1.0f32);
		match self {
			Layout::Operation{op, dir, sense: my_sense} => {
				*op = LayoutOperation::Split{proportion, first: Box::<Layout>::new(Layout::new()), second: Box::<Layout>::new(Layout::new())};
				*dir = direction;
				*my_sense = sense;
				match op {
					LayoutOperation::Split{proportion: _, first, second} => Ok((first.as_mut(), second.as_mut())),
					_ => unreachable!()
				}
			},
			_ => Err("Cannot be used with a item assigned to a Control")
		}
	}
	
	/// Divides the node into a given number of nodes with the same size
	pub fn divide(&mut self, divisions: usize, direction: Direction, sense: Sense) -> Result<&mut [Layout], &str>	{
		match self {
			Layout::Operation{op, dir, sense: my_sense} => {
				let mut children = Vec::with_capacity(divisions);
				for _i in 0..divisions{
					children.push(Layout::new());
				}

				*op = LayoutOperation::Divide(children);
				*dir = direction;
				*my_sense = sense;
				match op {
					LayoutOperation::Divide(elems) => Ok(elems.as_mut_slice()),
					_ => unreachable!()
				}
			},
			_ => Err("Cannot be used with a item assigned to a Control")
		}
		
	}

	/// Divides this node into two nodes, the former with a specific length in pixels
	pub fn trim(&mut self, pixels: u32, direction: Direction, sense: Sense) -> Result<(&mut Layout, &mut Layout), &str> {
		match self {
			Layout::Operation{op, dir, sense: my_sense} => {
				*op = LayoutOperation::Trim{pixels, first: Box::<Layout>::new(Layout::new()), second: Box::<Layout>::new(Layout::new())};
				*dir = direction;
				*my_sense = sense;
				match op {
					LayoutOperation::Trim{pixels: _, first, second} => Ok((first.as_mut(), second.as_mut())),
					_ => unreachable!()
				}
			},
			_ => Err("Cannot be used with a item assigned to a Control")
		}		
	}

	pub fn set_panel(&mut self, pixels: u32, panel: Panel) -> &mut Layout {
		*self = Layout::Group{span: pixels, panel, content: Box::new(Layout::default())};
		match self {
			Layout::Group{span: _, panel: _, content} => content,
			_ => unreachable!()
		}
}

	/// Converts this node into a leaf with a control inside
	pub fn set_widget(&mut self, widget: Widget) {
		*self = Layout::Control(widget);
	}

	pub fn resize(&mut self, space: Space) {
		match self {
			Layout::Operation{op, dir, sense} => {
				match op {
					LayoutOperation::Nop => return,
					LayoutOperation::Split{proportion, first, second} => {
						let (first_sp, second_sp) = space.split(*proportion, *dir, *sense);
						first.resize(first_sp);
						second.resize(second_sp);
					},
					LayoutOperation::Divide(layouts) => {
						let layouts_sp = space.divide(layouts.len(), *dir, *sense);
						for (lay, sp) in layouts.iter_mut().zip(layouts_sp){
							lay.resize(sp);
						}
					},
					LayoutOperation::Trim{pixels, first, second} => {
						let (first_sp, second_sp) = space.trim(*pixels, *dir, *sense);
						first.resize(first_sp);
						second.resize(second_sp);
					},
				}
			},
			Layout::Group{span, panel, content} => {
				let content_space = space.shrink(*span, *span); 
				panel.set_space(space);
				content.resize(content_space);
			}
			Layout::Control(widget) => widget.set_space(space)
		}		

	}

	pub fn get_widgets_mut(&mut self) -> Vec<&mut Widget> {
		match self {
			Layout::Operation{op, dir: _dir, sense: _sense} => {
				match op {
					LayoutOperation::Nop => Vec::new(),
					LayoutOperation::Split{proportion: _proportion, first, second} => {
						let mut result = first.get_widgets_mut();
						result.append(&mut second.get_widgets_mut());
						result
					},
					LayoutOperation::Divide(layouts) => {
						let mut result = Vec::new();
						for lay in layouts{
							result.append(&mut lay.get_widgets_mut());
						}
						result
					},
					LayoutOperation::Trim{pixels: _pixels, first, second} => {
						let mut result = first.get_widgets_mut();
						result.append(&mut second.get_widgets_mut());
						result
					},
				}
			},
			Layout::Group{span: _span, panel: _panel, content} => content.get_widgets_mut(),
			Layout::Control(widget) => {
				let mut result = Vec::new();
				result.push(widget);
				result
			}

		}
	}

	pub fn get_widgets(&self) -> Vec<&Widget> {
		match self {
			Layout::Operation{op, dir: _dir, sense: _sense} => {
				match op {
					LayoutOperation::Nop => Vec::new(),
					LayoutOperation::Split{proportion: _prop, first, second} => {
						let mut result = first.get_widgets();
						result.append(&mut second.get_widgets());
						result
					},
					LayoutOperation::Divide(layouts) => {
						let mut result = Vec::new();
						for lay in layouts{
							result.append(&mut lay.get_widgets());
						}
						result
					},
					LayoutOperation::Trim{pixels: _pixels, first, second} => {
						let mut result = first.get_widgets();
						result.append(&mut second.get_widgets());
						result
					},
				}
			},
			Layout::Group{span: _span, panel: _panel, content} => content.get_widgets(),
			Layout::Control(widget) => {
				let mut result = Vec::new();
				result.push(widget);
				result
			}

		}
	}

	pub fn get_panels(&self) -> Vec<&Panel> {
		match self {
			Layout::Operation{op, dir: _dir, sense: _sense} => {
				match op {
					LayoutOperation::Nop => Vec::new(),
					LayoutOperation::Split{proportion: _prop, first, second} => {
						let mut result = first.get_panels();
						result.append(&mut second.get_panels());
						result
					},
					LayoutOperation::Divide(layouts) => {
						let mut result = Vec::new();
						for lay in layouts{
							result.append(&mut lay.get_panels());
						}
						result
					},
					LayoutOperation::Trim{pixels: _pixels, first, second} => {
						let mut result = first.get_panels();
						result.append(&mut second.get_panels());
						result
					},
				}
			},
			Layout::Group{span: _span, panel, content} => {
				let mut result = Vec::new();
				result.push(panel);
				result.append(&mut content.get_panels());
				result
			},
			Layout::Control(_) => Vec::new(),
		}
	}
}