use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::{WindowBuilder, Window};

use winit::event_loop::EventLoop;

use pixels::{SurfaceTexture, PixelsBuilder};
use pixels::wgpu::{PowerPreference, RequestAdapterOptions, Color};



pub struct WindowingState {
    pub window: Window,
    pub dpi: u32,

    pub context: pixels::Pixels,
    pub size: LogicalSize<u32>,
}

impl WindowingState {
    pub fn new<T>(event_loop: &EventLoop<T>, dpi: u32) -> Self {

        let window = WindowBuilder::new()
            .with_maximized(false)
            .with_title("ray tracing")
            .build(&event_loop).unwrap(); 
        

        let surface_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(100, 100, &window);
        
        let mut context = PixelsBuilder::new(100, 100, surface_texture)
            .request_adapter_options(RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .enable_vsync(false)
            .build().unwrap();

        context.set_clear_color(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });

        
        let mut renderer = Self {
            window,
            dpi,

            context,
            size: LogicalSize::default(),
        };

        renderer.resize(surface_size);

        renderer
    }
    
    
    pub fn render(&self) {
        
        self.context.render().unwrap();
        self.window.request_redraw();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {

        self.context.resize_surface(size.width, size.height);

        let size = LogicalSize::new(size.width / self.dpi, size.height / self.dpi);
        self.size = size;
      
        self.context.resize_buffer(size.width, size.height);
    }
}