use glium::{
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::ActiveEventLoop,
        window::{Window, WindowId},
    },
};

#[allow(dead_code)]
pub struct TreaxisApp {
    window: Window,
    display: Display<WindowSurface>,
}

impl TreaxisApp {
    pub fn new(title: String) {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .expect("Could not create an event loop!");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(&title)
            .build(&event_loop);

        let mut app = TreaxisApp { window, display };

        event_loop
            .run_app(&mut app)
            .expect("Could not run event loop!");
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
                println!("redraw requested!")
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}
