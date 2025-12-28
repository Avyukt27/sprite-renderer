use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    screen_width: u32,
    screen_height: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            pixels: None,
            screen_width: 600,
            screen_height: 600,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Sprite Renderer")
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        self.screen_width,
                        self.screen_height,
                    )),
            )
            .unwrap();

        let window = Arc::new(window);

        let surface_texture =
            SurfaceTexture::new(self.screen_width, self.screen_height, window.clone());
        let pixels = Pixels::new(self.screen_width, self.screen_height, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let x = (i % self.screen_width as usize) as u8;
                        let y = (i / self.screen_height as usize) as u8;

                        pixel[0] = x; // R
                        pixel[1] = y; // G
                        pixel[2] = 0; // B
                        pixel[3] = 255; // A
                    }
                    pixels.render().unwrap();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
