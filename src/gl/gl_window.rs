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

extern crate glium;

use crate::{Screen, Window, Color, Surface, Space, ScreenId, UpdateData, MouseStatus, KeyboardStatus};
use crate::gl::gl_surface::GLSurface;

/// OpenGL implementation for Window
pub struct GLWindow{
    event_loop: glium::glutin::event_loop::EventLoop<()>,
    display: Option<glium::Display>,
    program: Option<glium::Program>,
    program_textures: Option<glium::Program>,
    screens: Vec<Screen>,
    dimensions: (u32, u32)
}

impl GLWindow {
    /// Creates a Window based on OpenGL
    /// params:
    ///     * caption: Title of the window
    ///     * width: Width of the window
    ///     * height: Height of the window
    ///     * resizable: Whether the window can be resized
    pub fn new(caption: &str, width: u32, height: u32, resizable: bool) -> Self {

        let event_loop = glium::glutin::event_loop::EventLoop::new();
        let window_builder = glium::glutin::window::WindowBuilder::new()
            .with_title(caption)
            .with_inner_size(glium::glutin::dpi::PhysicalSize{width, height})
            .with_resizable(resizable);
        let context_builder = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec4 color;
        out vec4 v_color;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_color = color;
        }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in vec4 v_color;

            void main() {
                gl_FragColor = v_color;
            }
        "#;
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2  tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coords;
            out vec4 v_color;

            uniform sampler2D tex;

            void main() {
                v_color = texture(tex, v_tex_coords);
            }
        "#;
        let tex_program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        GLWindow {
            event_loop,
            display: Some(display),
            program: Some(program),
            program_textures : Some(tex_program),
            //surface: Some(GLSurface::new(&display, &program, &tex_program)),
            screens: Vec::new(),
            dimensions: (width, height)
        }
    }

}

impl Window for GLWindow {

    fn add_screen(&mut self, mut screen: Screen) -> ScreenId {
        screen.resize(Space::new(self.dimensions.0, self.dimensions.1));
        self.screens.push(screen);
        self.screens.len()
    }

    fn execute(mut self) {

        let event_loop = self.event_loop;
        let display = std::mem::replace(&mut self.display, None).unwrap();
        let program = std::mem::replace(&mut self.program, None).unwrap();
        let program_tex = std::mem::replace(&mut self.program_textures, None).unwrap();
        let mut screens = std::mem::replace(&mut self.screens, Vec::new());
        let current_screen: usize = 0;
        let mut update_data = UpdateData{mouse_position: (0_u32, 0_u32), mouse_status: MouseStatus::Idle};
        let mut dimensions = self.dimensions;


        event_loop.run(move |ev, _, control_flow| {
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_millis(40u64);
            *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    glium::glutin::event::WindowEvent::CursorMoved{ device_id, position, modifiers } => {
                        let my_position = (position.x as u32, position.y as u32);
                        update_data.mouse_position = my_position;
                    },
                    glium::glutin::event::WindowEvent::Resized( size ) => {
                        dimensions = (size.width, size.height);
                        if current_screen <= screens.len() {
                            screens[current_screen].resize(Space::new(size.width, size.height));
                        }
                    },
    
                    _ => return,
                },
                _ => (),
            }

            let mut surface = GLSurface::new(
                &display, 
                &program, 
                &program_tex);
            surface.clear(&Color{r: 0_f32, g: 0_f32, b: 0_f32, a: 1_f32});
            if current_screen <= screens.len() {
                screens[current_screen].update(&update_data);
                screens[current_screen].paint(&mut surface);
            }
        });
    }
}