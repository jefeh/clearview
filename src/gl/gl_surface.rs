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
use crate::Surface;
use crate::ImageId;
use crate::gl::gl_image::GLImage;

use glium::Surface as RawGlSurface;
use std::cmp;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
struct ColorVertex {
    color: [f32; 4],
}

implement_vertex!(ColorVertex, color);

#[derive(Copy, Clone)]
struct TextureVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(TextureVertex, position, tex_coords);


pub struct GLSurface <'a, 'b> {
    display: &'a glium::Display,
    raw_surface: Option<glium::Frame>,
    program: &'b glium::Program,
    tex_program: &'b glium::Program,
    dimensions: (u32, u32),
    textures: Vec<GLImage>
}

impl<'a, 'b> GLSurface<'a, 'b>  {
    pub fn new(display: &'a glium::Display, program: &'b glium::Program, tex_program: &'b glium::Program) -> GLSurface<'a, 'b> {
        let raw_surface = display.draw();
        let dimensions = raw_surface.get_dimensions();
        let textures = Vec::new();
        GLSurface{ display, raw_surface: Some(raw_surface), program, tex_program, dimensions, textures }
    }

    fn to_color_vertex(color: &Color) -> ColorVertex {
        ColorVertex{color: [color.r, color.g, color.b, color.a]}
    }

    fn to_vertex(x: f32, y: f32) -> Vertex {
        Vertex{position: [x, y]}
    }

    fn to_tex_vertex(x: f32, y: f32, tx: f32, ty: f32) -> TextureVertex {
        TextureVertex{position: [x, y], tex_coords: [tx, ty]}
    }
}

impl<'a, 'b> Surface for GLSurface<'a, 'b> {

    fn load_image(&mut self, location: &str) -> ImageId {
        let image = GLImage::new(location, &self.display);
        self.textures.push(image);
        self.textures.len() - 1
    }
    
    fn clear(&mut self, color: &Color)
    {
        self.raw_surface.as_mut().map(|x| x.clear_color(color.r, color.g, color.b, color.a));
    }

    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, line_color: &Color) {
        let vertices = [
            GLSurface::to_vertex(x1, y1),
            GLSurface::to_vertex(x2, y2)
        ];
        let colors = glium::VertexBuffer::new(self.display, &[GLSurface::to_color_vertex(line_color); 2]).unwrap(); 
        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
        if let Some(surf) = &mut self.raw_surface {
            surf.draw((&vertex_buffer, &colors), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        }
    }

    fn draw_rectangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, border_color: Option<&Color>, fill_color: Option<&Color>) {
        let vertices = [
            GLSurface::to_vertex(x1, y1),
            GLSurface::to_vertex(x2, y1),
            GLSurface::to_vertex(x2, y2),
            GLSurface::to_vertex(x1, y2),
            GLSurface::to_vertex(x1, y1),
        ];
        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        if let Some(surf) = &mut self.raw_surface {

            if let Some(fc) = fill_color {
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
                let colors = glium::VertexBuffer::new(self.display, &[GLSurface::to_color_vertex(fc); 5]).unwrap(); 
                surf.draw((&vertex_buffer, &colors), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }

            if let Some(bc) = border_color {
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
                let colors = glium::VertexBuffer::new(self.display, &[GLSurface::to_color_vertex(bc); 5]).unwrap(); 
                surf.draw((&vertex_buffer, &colors), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
        }
    }
    

    fn draw_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, border_color: Option<&Color>, fill_color: Option<&Color>){
        let vertices = [
            GLSurface::to_vertex(x1, y1),
            GLSurface::to_vertex(x2, y2),
            GLSurface::to_vertex(x3, y3),
            GLSurface::to_vertex(x1, y1),
        ];
        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        
        if let Some(surf) = &mut self.raw_surface {

            if let Some(fc) = fill_color {
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
                let colors = glium::VertexBuffer::new(self.display, &[GLSurface::to_color_vertex(fc); 4]).unwrap(); 
                surf.draw((&vertex_buffer, &colors), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
            if let Some(bc) = border_color {
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
                let colors = glium::VertexBuffer::new(self.display, &[GLSurface::to_color_vertex(bc); 4]).unwrap(); 
                surf.draw((&vertex_buffer, &colors), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
        }
    }
    

    fn draw_ellipse(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, border_color: Option<&Color>, fill_color: Option<&Color>) {
        let width = x2 - x1;
        let height = y2 - y1;
        let mut vertices = Vec::new();

        let num_segments = cmp::max((width * self.dimensions.0 as f32) as u32, (height * self.dimensions.1 as f32) as u32);
        let theta = 2f32 * std::f32::consts::PI / num_segments as f32; 
        let c = theta.cos();
        let s = theta.sin();
    
        let mut px = 1f32; 
        let mut py = 0f32;
        let rx = width * 0.5f32;
        let ry = height * 0.5f32;
        let cx = x1 + rx;
        let cy = y1 + ry;

        let mut t;
        for _ in 0 ..= num_segments  { 
            vertices.push(GLSurface::to_vertex(px * rx + cx, py * ry + cy));
    
            t = px;
            px = c * px - s * py;
            py = s * t + c * py;
        } 
        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        if let Some(surf) = &mut self.raw_surface {

            if let Some(fc) = fill_color {
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
                let colors = vec![GLSurface::to_color_vertex(fc); vertex_buffer.len()];
                let colors_buffer = glium::VertexBuffer::new(self.display, &colors).unwrap(); 
                surf.draw((&vertex_buffer, &colors_buffer), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
            if let Some(bc) = border_color {
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
                let colors = vec![GLSurface::to_color_vertex(bc); vertex_buffer.len()];
                let colors_buffer = glium::VertexBuffer::new(self.display, &colors).unwrap(); 
                surf.draw((&vertex_buffer, &colors_buffer), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            }
        }

    }

    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, border_color: Option<&Color>, fill_color: Option<&Color>) {
        self.draw_ellipse(x, y, radius, radius, border_color, fill_color);
    }

    fn draw_rounded_rectangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, rad_pixel: u32, border_color: Option<&Color>, fill_color: Option<&Color>) {
        /*
        let h_pad = (rad_prop.clamp(0.0_f32, 0.5_f32) * width as f32) as u32;
        let v_pad = (rad_prop.clamp(0.0_f32, 0.5_f32) * height as f32) as u32;
        let x1_prop = x + h_pad;
        let x2_prop = ((x + width) as i32 - h_pad as i32) as u32;
        let y1_prop = y + v_pad;
        let y2_prop = ((y + height) as i32 - v_pad as i32) as u32;

        let num_segments = cmp::max(h_pad, v_pad);
        let theta = 0.5_f32 * std::f32::consts::PI / num_segments as f32; 
        let c = theta.cos();
        let s = theta.sin();
        let rx = h_pad as f32;
        let ry = v_pad as f32;
        let mut t;
    
        let (r, g, b) = GLSurface::translate_color(border_color);
        let mut vertices = Vec::new();
        vertices.push(self.translate_point(x1_prop, y));
        vertices.push(self.translate_point(x2_prop, y));

        let mut px = 0_f32; 
        let mut py = -1_f32;
        let mut cx = x2_prop as f32;
        let mut cy = y1_prop as f32;
        for _ in 0 ..= num_segments  { 
            vertices.push(self.translate_point((px * rx + cx) as u32, (py * ry + cy) as u32)); 

            t = px;
            px = c * px - s * py;
            py = s * t + c * py;
        } 

        vertices.push(self.translate_point(x + width, y1_prop));
        vertices.push(self.translate_point(x + width, y2_prop));

        px = 1_f32; 
        py = 0_f32;
        cx = x2_prop as f32;
        cy = y2_prop as f32;
        for _ in 0 ..= num_segments  { 
            vertices.push(self.translate_point((px * rx + cx) as u32, (py * ry + cy) as u32)); 

            t = px;
            px = c * px - s * py;
            py = s * t + c * py;
        } 

        vertices.push(self.translate_point(x2_prop, y + height));
        vertices.push(self.translate_point(x1_prop, y + height));

        px = 0_f32; 
        py = 1_f32;
        cx = x1_prop as f32;
        cy = y2_prop as f32;
        for _ in 0 ..= num_segments  { 
            vertices.push(self.translate_point((px * rx + cx) as u32, (py * ry + cy) as u32)); 

            t = px;
            px = c * px - s * py;
            py = s * t + c * py;
        } 

        vertices.push(self.translate_point(x, y2_prop));
        vertices.push(self.translate_point(x, y1_prop));

        px = -1_f32; 
        py = 0_f32;
        cx = x1_prop as f32;
        cy = y1_prop as f32;
        for _ in 0 ..= num_segments  { 
            vertices.push(self.translate_point((px * rx + cx) as u32, (py * ry + cy) as u32)); 

            t = px;
            px = c * px - s * py;
            py = s * t + c * py;
        } 

        vertices.push(self.translate_point(x1_prop, y));

        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        let colors = vec![ColorVertex { color: [r, g, b] }; vertex_buffer.len()];
        let colors_buffer = glium::VertexBuffer::new(self.display, &colors).unwrap(); 
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
        if let Some(surf) = &mut self.raw_surface {
            surf.draw((&vertex_buffer, &colors_buffer), &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        }
        */
    }


    fn draw_text(&mut self, text: &str, x1: f32, y1: f32, x2: f32, y2: f32, text_color: &Color) {

    }

    fn draw_image(&mut self, img: ImageId, x1: f32, y1: f32, x2: f32, y2: f32) {
        if img >= self.textures.len() {
            return;
        }

        let vertices = [
            GLSurface::to_tex_vertex(x1, y1, 0_f32, 1_f32),
            GLSurface::to_tex_vertex(x2, y1, 1_f32, 1_f32),
            GLSurface::to_tex_vertex(x2, y2, 1_f32, 0_f32),
            GLSurface::to_tex_vertex(x1, y2, 0_f32, 0_f32),
        ];
        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        if let Some(surf) = &mut self.raw_surface {
            let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0, 1.0_f32] ],
                tex: &self.textures[img].texture
            };
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
            surf.draw(&vertex_buffer, &indices, &self.tex_program, &uniforms, &Default::default()).unwrap();
        }

    }
}

impl<'a, 'b> Drop for GLSurface<'a, 'b> {
    fn drop(&mut self) {
        self.raw_surface.take().unwrap().finish().unwrap();
    }
}