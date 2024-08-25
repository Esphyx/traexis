use glium::{
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::ActiveEventLoop,
        keyboard::{KeyCode, PhysicalKey},
        window::{Fullscreen, Window, WindowId},
    },
    Surface,
};

use super::camera::Camera;

#[allow(dead_code)]
pub struct TreaxisApp {
    window: Window,
    display: Display<WindowSurface>,
    camera: Camera,
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
        };

        event_loop
            .run_app(&mut app)
            .expect("Could not run the event loop!");
    }
}

impl ApplicationHandler for TreaxisApp {
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
                let mut target = self.display.draw();
                let size: (u32, u32) = self.window.inner_size().into();
                println!("{:?}", size);

                target.clear_color(0.0, 0.5, 0.5, 1.0);
                target.finish().expect("Could not finish drawing frame!");
            }
            WindowEvent::Resized(new_size) => {
                self.display.resize((new_size.width, new_size.height));
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    match keycode {
                        KeyCode::Escape => {
                            event_loop.exit();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
