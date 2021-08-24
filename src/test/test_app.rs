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

use crate::create_window;
use crate::Window;
use crate::{Layout, Direction, Sense};
use crate::Widget;
use crate::Screen;
use crate::Color;
use crate::Panel;
use crate::Theme;

/*struct MyApp {

}*/

pub fn main() {
    let mut window = create_window("Test", 800_u32, 600_u32, true);

    window.add_screen(Screen::new(main_screen));
    window.execute();

}

pub fn main_toolbar(layout: &mut Layout) -> &mut Layout {
    let theme = Theme::new();
    let signal = 0u32;

    let (toolbar_sp, remaining_sp) = layout.trim(158_u32, Direction::Vertical, Sense::Forward).unwrap();
    let toolbar_content = toolbar_sp.set_panel(15_u32, Panel::new(theme.background, None));

    let (button, remaining_buttons) = toolbar_content.trim(128u32, Direction::Horizontal, Sense::Forward).unwrap();
    button.set_widget(Widget::new_button(signal, theme));
    let (button, remaining_buttons) = remaining_buttons.trim(128u32, Direction::Horizontal, Sense::Forward).unwrap();
    button.set_widget(Widget::new_button(signal, theme));
    let (button, remaining_buttons) = remaining_buttons.trim(128u32, Direction::Horizontal, Sense::Forward).unwrap();
    button.set_widget(Widget::new_button(signal, theme));
    let (button, remaining_buttons) = remaining_buttons.trim(128u32, Direction::Horizontal, Sense::Forward).unwrap();
    button.set_widget(Widget::new_button(signal, theme));
    let (button, _remaining_buttons) = remaining_buttons.trim(128u32, Direction::Horizontal, Sense::Forward).unwrap();
    button.set_widget(Widget::new_button(signal, theme));
    remaining_sp
}

pub fn main_screen(layout: &mut Layout) {
    let theme = Theme::new();
    let signal = 0u32;
    let client_sp = main_toolbar(layout);
    let (button_sp, second_sp) = client_sp.trim(100_u32, Direction::Horizontal, Sense::Forward).unwrap();
    button_sp.set_widget(Widget::new_button(signal, theme));
    let (_first_sp, second_but_sp) = second_sp.trim(200_u32, Direction::Vertical, Sense::Forward).unwrap();
    second_but_sp.set_widget(Widget::new_button(signal, theme));
}