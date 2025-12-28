use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::colour::Colour;
use crate::renderer::Renderer;
use crate::sprite::Sprite;

pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    renderer: Renderer,
    sprites: Vec<Sprite>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            pixels: None,
            renderer: Renderer::new(600, 600),
            sprites: Vec::new(),
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
                        self.renderer.width,
                        self.renderer.height,
                    )),
            )
            .unwrap();

        let window = Arc::new(window);

        let surface_texture =
            SurfaceTexture::new(self.renderer.width, self.renderer.height, window.clone());
        let pixels =
            Pixels::new(self.renderer.width, self.renderer.height, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);

        self.sprites.clear();
        let sprites = vec![
            Sprite::new(50, 50, Colour::new(255, 0, 0, 255), 8, 8),
            Sprite::new(100, 100, Colour::new(0, 255, 0, 255), 16, 16),
            Sprite::new(150, 150, Colour::new(0, 0, 255, 255), 8, 16),
        ];
        self.sprites.extend(sprites);
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
                self.renderer.clear(Colour::new(0, 0, 0, 255));
                for sprite in &self.sprites {
                    self.renderer.draw_sprite(sprite);
                }

                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    frame.copy_from_slice(&self.renderer.buffer);
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
