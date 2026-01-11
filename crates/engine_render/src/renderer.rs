use winit::window::Window;

/// Renderer is responsible for initializing GPU state and drawing frames.
///
/// Right now it's just a stub so the workspace compiles and you can see
/// when frames are being rendered
pub struct Renderer {}

impl Renderer {
    /// Create a new Renderer.
    /// Later, you will pass in a window handle or surface here.
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        println!(
            "Renderer created for window size: {}x{}",
            size.width, size.height
        );
        Renderer {}
    }

    /// Draw a single frame.
    /// Later, this will issue wgpu commands to clear the screen and draw.
    pub fn render_frame(&mut self) {
        println!("Renderer::render_frame called (stub)")
    }
}
