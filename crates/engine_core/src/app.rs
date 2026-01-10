use engine_render::Renderer;

/// Engine is the main entry point for running a game.
/// For now, it just creates a Renderer stub and exists.
pub struct Engine {
    renderer: Renderer,
    // Later: fields for window, ECS world, input, timing, etc.
}

impl Engine {
    /// Build a new Engine instance.
    /// Later, you may pass configuration, window settings, etc.
    pub fn new() -> Self {
        let renderer = Renderer::new();

        Engine { renderer }
    }

    /// Run the engine main loop.
    /// For now, this just renders a fixed number of frames and exists
    /// Step 5 will replace this with a proper event loop and game loop
    pub fn run (&mut self) {
        // Temporary stub main loop: render 3 frames then exit
        for _ in 0..3 {
            self.renderer.render_frame();
            println!("Engine frame rendered (stub).");
        }
    }
}