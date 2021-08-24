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

use crate::{Surface, Space, UpdateData, Layout, Panel};

pub struct Screen {
    layout: Layout,
}

impl Screen {
    pub fn new(init_function: fn(&mut Layout)) -> Self {
        let mut layout = Layout::new();
        init_function(&mut layout);
        Screen{layout}
    }

    pub fn resize(&mut self, space: Space) {
        self.layout.resize(space);
    }

    pub fn update(&mut self, data: &UpdateData) {
        self.layout.get_widgets_mut().iter_mut().map(|x| x.update(data)).collect()
    }

    pub fn paint(&self, surface: &mut dyn Surface) {
        self.layout.get_panels().iter().map(|x| x.paint(surface)).for_each(drop);
        self.layout.get_widgets().iter().map(|x| x.paint(surface)).collect()
    }
}