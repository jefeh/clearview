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

mod button;

use crate::Space;
use crate::UpdateData;
use crate::Surface;
use crate::Color;
use crate::Theme;

use button::paint_button;

enum WidgetInfo {
    None,
    Text(String),
    Number(i32),
    Float(f32),
    Image()
}

enum WidgetStatus {
    Inactive,
    Active,
    Disabled,
    Hover,
}

pub struct Widget {
    space: Space,
    theme: Theme,
    info: WidgetInfo,
    color_fg: Color,
    color_bg: Color,
    color_border: Color,
    status: WidgetStatus,
    update_fn: Option<fn(&UpdateData)>,
    paint_fn: Option<fn(&Space, &mut dyn Surface, color_fg: &Color, color_bg: &Color, color_border: &Color)>,
}

impl std::fmt::Debug for Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Control {:?})", self.space)
    }
}

impl Widget {
    pub fn new_button(signal: u32, theme: Theme) -> Self {
        Widget{
            space: Space::new(1, 1), 
            theme, 
            info: WidgetInfo::None,
            color_fg: theme.border,
            color_bg: theme.inactive,
            color_border: theme.border,
            status: WidgetStatus::Inactive,
            update_fn: None, 
            paint_fn: Some(button::paint_button)
        }
    }

    pub fn set_space(&mut self, space: Space) {
        self.space = space;
    }

    pub fn update(&mut self, data: &UpdateData) {
        self.status = if self.space.is_inside(data.mouse_position.0, data.mouse_position.1) {
            WidgetStatus::Hover
        } else { 
            WidgetStatus::Inactive
        };

        match self.status {
            WidgetStatus::Inactive => {
                self.color_fg.approach(self.theme.border);
                self.color_bg.approach(self.theme.inactive);
                self.color_border.approach(self.theme.border);
            },
            WidgetStatus::Active => {
                self.color_fg.approach(self.theme.border);
                self.color_bg.approach(self.theme.active);
                self.color_border.approach(self.theme.border);
            },
            WidgetStatus::Disabled => {
                self.color_fg.approach(self.theme.border);
                self.color_bg.approach(self.theme.inactive);
                self.color_border.approach(self.theme.border);
            },
            WidgetStatus::Hover => {
                self.color_fg.approach(self.theme.border);
                self.color_bg.approach(self.theme.hover);
                self.color_border.approach(self.theme.border);
            },
        }
    }

    pub fn paint(&self, surface: &mut dyn Surface) {
        if let Some(function) = self.paint_fn {
            function(&self.space, surface, &self.theme.active, &self.color_bg, &self.color_border);
        }       
    }
}

#[derive(Debug)]
pub struct Panel {
    space: Space,
    color: Color,
    border_color: Option<Color>,
}

impl Panel {
    pub fn new(color: Color, border_color: Option<Color>) -> Self {
        Panel{space: Space::new(1, 1), color, border_color}
    }

    pub fn set_space(&mut self, space: Space) {
        self.space = space;
    }

    pub fn paint(&self, surface: &mut dyn Surface) {
        surface.draw_rectangle(
            self.space.prop_coords.0, self.space.prop_coords.1, 
            self.space.prop_coords.2, self.space.prop_coords.3, 
            self.border_color.as_ref(), Some(&self.color));       
    }
}