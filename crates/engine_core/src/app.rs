use engine_render::Renderer;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

use winit::dpi::LogicalSize;

use crate::{EngineConfig, Time};

use log::{debug, info};

/// Main engine state owned by the event loop.
pub struct Engine {
    window: Option<Window>,
    renderer: Option<Renderer>,
    time: Time,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            time: Time::new(),
        }
    }

    fn init_window(&mut self, ael: &ActiveEventLoop) {
        let config = EngineConfig::load();

        info!("Creating window");
        let window = ael
            .create_window(
                WindowAttributes::default()
                    .with_title(config.window.title)
                    .with_inner_size(LogicalSize::new(
                        config.window.width as f64,
                        config.window.height as f64,
                    )),
            )
            .unwrap();

        info!("Initializing renderer");
        let renderer = Renderer::new(&window);

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn update(&mut self) {
        self.time.update();
    }

    fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            renderer.render_frame();
        }
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.init_window(event_loop);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                println!("Window resized to {}x{}", size.width, size.height);
            }

            WindowEvent::RedrawRequested => {
                self.render();
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.update();

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

/// Entry point
pub fn run() {
    info!("Engine run() starting");
    let event_loop = EventLoop::new().unwrap();
    let mut engine = Engine::new();

    event_loop.run_app(&mut engine).unwrap();
}
