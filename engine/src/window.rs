use glium::glutin::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{self, WindowBuilder, Fullscreen},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};

pub struct WindowSettings<'a> {
    width: u32,
    height: u32,
    title: &'a str,
    fullscreen: bool,
}

impl Default for WindowSettings<'_> {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "window",
            fullscreen: false,
        }
    }
}

pub struct Window {
    context: ContextWrapper<PossiblyCurrent, window::Window>,
}

fn create_window_builder(settings: &WindowSettings) -> WindowBuilder {
    if settings.fullscreen {
        let event_loop = EventLoop::new();
        let monitor = event_loop.primary_monitor();

        WindowBuilder::new()
            .with_inner_size(PhysicalSize {
                width: settings.width,
                height: settings.height,
            })
            .with_title(settings.title)
            .with_fullscreen(Some(Fullscreen::Borderless(monitor)))
    } else {
        WindowBuilder::new()
            .with_inner_size(PhysicalSize {
                width: settings.width,
                height: settings.height,
            })
            .with_title(settings.title)
    }
}

impl Window {
    pub fn new(settings: WindowSettings, event_loop: &EventLoop<()>) -> Self {
        let window_builder = create_window_builder(&settings);
        let context = ContextBuilder::new()
            .build_windowed(window_builder, event_loop)
            .unwrap();

        let context = unsafe { context.make_current().unwrap() };

        Self { context }
    }
}
