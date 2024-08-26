use glium::{
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler,
        dpi::{LogicalPosition, PhysicalPosition},
        event::WindowEvent,
        event_loop::ActiveEventLoop,
        keyboard::{KeyCode, PhysicalKey},
        window::{Fullscreen, Window, WindowId},
    },
    Surface,
};
use rand::Rng;

use crate::graphics::{
    linear_algebra::{normalize, rotate_x, rotate_y, scale},
    vertex::Vertex,
};

use super::camera::Camera;

#[allow(dead_code)]
pub struct TreaxisApp {
    window: Window,
    display: Display<WindowSurface>,
    camera: Camera,
    cursor_lock: bool,
    mouse_position: Option<PhysicalPosition<f64>>,
    mouse_delta: (f32, f32),
}

impl TreaxisApp {
    pub fn new(title: String, fullscreen: bool) {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("Could not create an event loop!");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(&title)
            .build(&event_loop);

        if fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(window.primary_monitor())));
        }

        let mut app = TreaxisApp {
            window,
            display,
            camera: Camera::default(),
            cursor_lock: true,
            mouse_position: None,
            mouse_delta: (0.0, 0.0),
        };

        event_loop
            .run_app(&mut app)
            .expect("Could not run the event loop!");
    }
}

impl ApplicationHandler for TreaxisApp {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        println!("resumed!");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let (delta_x, _) = self.mouse_delta;
                self.camera.set_direction(rotate_y(
                    self.camera.get_direction(),
                    delta_x as f32 * -0.01,
                ));
                self.mouse_delta = (0.0, 0.0);

                if let Some(pos) = self.mouse_position {
                    self.window
                        .set_cursor_position(pos)
                        .expect("Unable to set cursor position!");
                }

                let mut target = self.display.draw();
                let (width, height) = target.get_dimensions();

                let shape = vec![
                    Vertex {
                        position: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 0.0, 1.0],
                    },
                ];

                let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape)
                    .expect("Could not create the vertex buffer!");
                let indices = glium::IndexBuffer::new(
                    &self.display,
                    glium::index::PrimitiveType::TrianglesList,
                    &[0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                )
                .expect("Could not create index buffer!");

                let program = glium::Program::from_source(
                    &self.display,
                    include_str!("shaders/vertex_shader.glsl"),
                    include_str!("shaders/fragment_shader.glsl"),
                    None,
                )
                .expect("Could not compile program!");

                let draw_parameters = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let light = [1.0f32, 1.0, 1.0];
                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &glium::uniform! {
                            light: light,
                            view: self.camera.view_matrix(),
                            perspective: self.camera.perspective(width, height)
                        },
                        &draw_parameters,
                    )
                    .expect("Could not draw");
                target.finish().expect("Could not finish drawing frame!");
            }
            WindowEvent::Resized(new_size) => {
                self.display.resize((new_size.width, new_size.height));
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.cursor_lock {
                    if let Some(previous) = self.mouse_position {
                        let (previous_x, previous_y): (f64, f64) = previous.into();
                        let (x, y): (f64, f64) = position.into();

                        let (delta_x, delta_y) = (x - previous_x, y - previous_y);
                        println!(
                            "cursor locked: {} delta({}, {})",
                            self.cursor_lock, delta_x, delta_y
                        );
                        self.mouse_delta = (delta_x as f32, delta_y as f32);
                    }
                }
                self.mouse_position = Some(position);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    const DELTA: f32 = 0.01;
                    match keycode {
                        KeyCode::Escape => event_loop.exit(),
                        KeyCode::KeyW => {
                            self.camera
                                .mv(scale(normalize(self.camera.forward()), DELTA));
                        }
                        KeyCode::KeyA => {
                            self.camera.mv(scale(normalize(self.camera.left()), DELTA));
                        }
                        KeyCode::KeyS => {
                            self.camera.mv(scale(normalize(self.camera.back()), DELTA));
                        }
                        KeyCode::KeyD => {
                            self.camera.mv(scale(normalize(self.camera.right()), DELTA));
                        }
                        KeyCode::Space => {
                            self.camera.eye[1] += DELTA;
                            self.camera.target[1] += DELTA;
                        }
                        KeyCode::ShiftLeft => {
                            self.camera.eye[1] -= DELTA;
                            self.camera.target[1] -= DELTA;
                        }
                        KeyCode::Comma => self.camera.fov -= DELTA,
                        KeyCode::Period => self.camera.fov += DELTA,
                        KeyCode::ArrowUp => self
                            .camera
                            .set_direction(rotate_x(self.camera.get_direction(), DELTA)),
                        KeyCode::ArrowLeft => self
                            .camera
                            .set_direction(rotate_y(self.camera.get_direction(), -DELTA)),
                        KeyCode::ArrowDown => self
                            .camera
                            .set_direction(rotate_x(self.camera.get_direction(), -DELTA)),
                        KeyCode::ArrowRight => self
                            .camera
                            .set_direction(rotate_y(self.camera.get_direction(), DELTA)),
                        KeyCode::KeyL => {
                            self.cursor_lock = !self.cursor_lock;
                            // self.window.set_cursor_visible(self.cursor_lock);
                            if self.cursor_lock {
                                self.window
                                    .set_cursor_grab(glium::winit::window::CursorGrabMode::Confined)
                                    .expect("Couldn't set cursor grab mode!");
                            } else {
                                self.window
                                    .set_cursor_grab(glium::winit::window::CursorGrabMode::None)
                                    .expect("Couldn't set cursor grab mode!");
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
