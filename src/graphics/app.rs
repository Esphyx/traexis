use core::f32;

use glium::{
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    index::{NoIndices, PrimitiveType},
    winit::{
        application::ApplicationHandler,
        dpi::PhysicalPosition,
        event::{DeviceEvent, ElementState, RawKeyEvent, WindowEvent},
        event_loop::ActiveEventLoop,
        keyboard::{KeyCode, PhysicalKey},
        window::{Fullscreen, Window, WindowId},
    },
    Surface,
};
use treaxis_core::State;

use crate::{
    graphics::{
        linear_algebra::{normalize, scale},
        vertex::Renderable,
    },
    DEPTH, HEIGHT, WIDTH,
};

use super::camera::Camera;

#[allow(dead_code)]
pub struct TreaxisApp {
    window: Window,
    display: Display<WindowSurface>,
    state: State<{ crate::WIDTH }, { crate::HEIGHT }, { crate::DEPTH }>,
    camera: Camera,
    delta: (f64, f64),
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
            camera: Default::default(),
            delta: Default::default(),
            state: Default::default(),
        };

        let current = &app.state.current;
        let shape = current.get_shape();
        for (x, r) in shape.iter().enumerate() {
            for (y, c) in r.iter().enumerate() {
                for (z, b) in c.iter().enumerate() {
                    if !b {
                        continue;
                    }
                    let [px, py, pz] = current.position;

                    let [ax, ay, az] = [px + x, py + y, pz + z];
                    if ax < WIDTH && ay < HEIGHT && az < DEPTH {
                        app.state.playfield[ay].set(x, z);
                    }
                }
            }
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
}

impl ApplicationHandler for TreaxisApp {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: glium::winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.delta = delta;
                let (width, height): (f64, f64) = self.window.inner_size().into();
                self.window
                    .set_cursor_position(PhysicalPosition::new(
                        width as f64 / 2.0,
                        height as f64 / 2.0,
                    ))
                    .expect("Could not set cursor position!");
            }
            DeviceEvent::Key(key_event) => {
                let RawKeyEvent {
                    physical_key,
                    state,
                } = key_event;

                match state {
                    ElementState::Pressed => match physical_key {
                        PhysicalKey::Code(keycode) => match keycode {
                            KeyCode::F11 => self.toggle_fullscreen(),
                            _ => {}
                        },
                        _ => {}
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
                let mut target = self.display.draw();
                let (width, height) = target.get_dimensions();

                let (delta_x, delta_y) = self.delta;

                self.camera
                    .process_mouse(delta_x as f32 * 0.01, delta_y as f32 * 0.01);
                self.delta = (0.0, 0.0);

                let shape = self.state.to_vertices();

                let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape)
                    .expect("Could not create the vertex buffer!");
                let indices = NoIndices(PrimitiveType::TrianglesList);

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

                target.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &glium::uniform! {
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
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    const DELTA: f32 = 0.05;
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
                        KeyCode::Space => self.camera.mv([0.0, DELTA, 0.0]),
                        KeyCode::ShiftLeft => self.camera.mv([0.0, -DELTA, 0.0]),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
