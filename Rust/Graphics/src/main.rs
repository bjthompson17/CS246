#![windows_subsystem="windows"]

extern crate glium;

use autopilot::mouse;
use autopilot::screen;
use glium::{
    glutin::dpi::*,
    Display,
    Surface,
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        ContextBuilder,
        window::WindowBuilder
    },
};

use std::time::{Instant,Duration};

// window size for glutin/winit============================================
 const WINDOW_SIZE:PhysicalSize<i32> = PhysicalSize::new(20, 20);
 const FPS:u64 = 60;
 const LOW_FRICTION:f64 = 0.995;
 const HIGH_FRICTION:f64 = 0.7;

fn main() {

    //window setup==============================================

    let events_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_transparent(true)
        .with_inner_size(WINDOW_SIZE)
        .with_decorations(false)
        .with_always_on_top(true)
        .with_resizable(false)
        ;

    let cb = ContextBuilder::new();

    let mut app = App::new();
    let display = Display::new(wb,cb,&events_loop).unwrap();

    // image prep ================================================
    use glium::{
        index::PrimitiveType,
        program
    };

    let image = image::open("images\\Awesomeface.png").unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(&display, image).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        glium::implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
            ]
        ).unwrap()
    };

    // building the index buffer, includes data on how vertecies are drawn
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip,
                                               &[1 as u16, 2, 0, 3]).unwrap();

    // GLSL Shaders for OpenGl version 3.1, tells GPU how to draw the image
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
        }).unwrap();

    // set drawing parameters, such as blend mode for transparency
    let draw_params = glium::DrawParameters {
        blend:glium::Blend::alpha_blending(),
        ..Default::default()
    };
        
    //Event Loop =================================================

    let mut start_time = Instant::now();

    events_loop.run(move |event, _, control_flow| {
        match *control_flow {
            ControlFlow::Exit => (),
            _ => {
                let mut next_inst = Instant::now();
                let elapsed_time = next_inst.duration_since(start_time).as_millis() as u64;
                if elapsed_time < 1000 / FPS {
                    next_inst += Duration::from_millis((1000 / FPS) - elapsed_time);
                    *control_flow = ControlFlow::WaitUntil(next_inst);
                } else {
                    //This code is FPS limited for consistant motion
                    start_time = Instant::now();
                    display.gl_window().window().request_redraw();
                    app.update();
                    display.gl_window().window().set_outer_position(
                        PhysicalPosition::new(
                            app.x as i32 - (WINDOW_SIZE.width as i32 / 2),
                            app.y as i32 - (WINDOW_SIZE.height as i32 / 2)
                        ));
                    if  (mouse::location().x as i32) > (app.x as i32 - WINDOW_SIZE.width / 2) &&
                        (mouse::location().x as i32) < (app.x as i32 + WINDOW_SIZE.width / 2)  &&
                        (mouse::location().y as i32) > (app.y as i32 - WINDOW_SIZE.height / 2) &&
                        (mouse::location().y as i32) < (app.y as i32 + WINDOW_SIZE.height / 2) 
                    {
                        //display.gl_window().window().set_visible(false);
                    }else {
                        //display.gl_window().window().set_visible(true);
                    }
                }
                
            }

        }

        match event {
            Event::LoopDestroyed => return,

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let mut target = display.draw();
                target.clear_color_srgb(0.0, 0.0, 0.0, 0.0);
                // draw stuff
                let uniforms = glium::uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                    ],
                    tex: &opengl_texture
                };

                target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &draw_params).unwrap();
                target.finish().unwrap();
            }
            _ => (),
        }
    });
}



struct App {
    x:f64,
    y:f64,
    vx:f64,
    vy:f64
}

impl App {
    fn new() -> App {
        App {
            x:screen::size().width / 2.0,
            y:screen::size().height / 2.0,
            vx:0.0,
            vy:0.0
        }
    }

    fn update(&mut self) {
        let mouse_pos = mouse::location();
        let mouse_x:i32 = mouse_pos.x as i32;
        let mouse_y:i32 = mouse_pos.y as i32;
        let vector:[f64; 2] = [ mouse_x as f64 - self.x, mouse_y as f64 - self.y];
        let dist = (vector[0].powi(2) + vector[1].powi(2)).sqrt();
        let range = ((WINDOW_SIZE.width + WINDOW_SIZE.height) / 2) as f64;
        if dist > range { 
            self.vx += (vector[0] / dist) * 100.0/(dist - range + 25.0);
            self.vy += (vector[1] / dist) * 100.0/(dist - range + 25.0);
        } else {
            self.vx *= HIGH_FRICTION;
            self.vy *= HIGH_FRICTION;
        }
        self.vx *= LOW_FRICTION;
        self.vy *= LOW_FRICTION;
        self.x += self.vx;
        self.y += self.vy;
        
        if self.x <= 0.0 {
            self.x += screen::size().width;
        } else if self.x > screen::size().width {
            self.x -= screen::size().width;
        }

        if self.y < 0.0 {
            self.y += screen::size().height;
        } else if self.y > screen::size().height {
            self.y -= screen::size().height;
        }
    }
}
