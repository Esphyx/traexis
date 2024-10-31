use core::f32;
use std::{collections::HashSet, time::Instant};

use glium::{
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    index::{NoIndices, PrimitiveType},
    winit::{
        application::ApplicationHandler,
        dpi::PhysicalPosition,
        event::*,
        event_loop::ActiveEventLoop,
        keyboard::{KeyCode, PhysicalKey},
        window::*,
    },
    Program, Surface, VertexBuffer,
};
use traexis_core::State;

use crate::client::{
    axes::get_axes, grid_lines::get_grid_lines, renderable::Renderable, vertex::Vertex,
};

use super::client::camera::Camera;

#[allow(dead_code)]
pub struct App {
    window: Window,
    display: Display<WindowSurface>,
    state: State<{ crate::WIDTH }, { crate::HEIGHT }, { crate::DEPTH }>,
    camera: Camera,
    keys: HashSet<KeyCode>,
    last_time: Instant,
}

impl App {
    pub fn new(title: String, fullscreen: bool) {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("Could not create an event loop!");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(&title)
            .build(&event_loop);

        let mut app = App {
            window,
            display,
            camera: Default::default(),
            state: Default::default(),
            keys: HashSet::new(),
            last_time: Instant::now(),
        };

        if fullscreen {
            app.toggle_fullscreen();
        }

        app.window.set_cursor_visible(false);

        event_loop
            .run_app(&mut app)
            .expect("Could not run the event loop!");
    }

    pub fn toggle_fullscreen(&mut self) {
        self.window.set_fullscreen(
            self.window
                .fullscreen()
                .is_none()
                .then(|| Fullscreen::Borderless(self.window.primary_monitor())),
        );
    }

    pub fn process_input(&mut self) {
        self.camera.process_keys(&self.keys);
    }
}

impl ApplicationHandler for App {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: glium::winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let (delta_x, delta_y) = delta;
                let (width, height): (f64, f64) = self.window.inner_size().into();
                self.window
                    .set_cursor_position(PhysicalPosition::new(
                        width as f64 / 2.0,
                        height as f64 / 2.0,
                    ))
                    .expect("Could not set cursor position!");

                self.camera
                    .process_mouse(delta_x as f32 * 0.001, delta_y as f32 * 0.001);
            }
            DeviceEvent::Key(key_event) => {
                let RawKeyEvent {
                    physical_key,
                    state,
                } = key_event;

                match physical_key {
                    PhysicalKey::Code(keycode) => match state {
                        ElementState::Pressed => {
                            self.keys.insert(keycode);
                            match keycode {
                                KeyCode::F11 => self.toggle_fullscreen(),
                                KeyCode::Escape => event_loop.exit(),
                                _ => {}
                            }
                        }
                        ElementState::Released => {
                            self.keys.remove(&keycode);
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let current_time = Instant::now();
                let delta_time = current_time.duration_since(self.last_time).as_secs_f32();
                self.last_time = current_time;

                self.process_input();

                self.camera.update(delta_time);

                self.state.clear();
                let current = &self.state.current;
                let shape = current.get_shape();
                self.state.merge(shape, (0, 0, 0));

                let mut target = self.display.draw();

                let (width, height) = target.get_dimensions();

                target.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);

                // SETUP RENDERING
                let program = Program::from_source(
                    &self.display,
                    include_str!("client/shaders/vertex_shader.glsl"),
                    include_str!("client/shaders/fragment_shader.glsl"),
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

                let uniforms = glium::uniform! {
                    width: crate::WIDTH as f32,
                    height: crate::HEIGHT as f32,
                    depth: crate::DEPTH as f32,
                    view: self.camera.view_matrix(),
                    perspective: self.camera.perspective(width, height)
                };
                // ---------------

                // DRAW AXES
                #[cfg(debug_assertions)]
                #[allow(unused)]
                {
                    let vertex_buffer = VertexBuffer::new(&self.display, &get_axes())
                        .expect("Could not create a vertex buffer!");
                    let indices = NoIndices(PrimitiveType::LinesList);

                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &draw_parameters,
                        )
                        .expect("Could not draw");
                }

                // DRAW GRID LINES
                let vertex_buffer = VertexBuffer::new(&self.display, &get_grid_lines())
                    .expect("Could not create a vertex buffer!");
                let indices = NoIndices(PrimitiveType::LinesList);
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &draw_parameters,
                    )
                    .expect("Could not draw!");

                // DRAW GARBAGE
                let vertex_buffer = VertexBuffer::new(&self.display, &self.state.to_vertices())
                    .expect("Could not create the vertex buffer!");
                let indices = NoIndices(PrimitiveType::TrianglesList);

                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &draw_parameters,
                    )
                    .expect("Could not draw");
                target.finish().expect("Could not finish drawing frame!");
            }
            WindowEvent::Resized(new_size) => {
                self.display.resize((new_size.width, new_size.height));
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}
