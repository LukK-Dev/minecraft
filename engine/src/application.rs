use glium::glutin::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    renderer::Renderer,
    window::{Window, WindowSettings},
};

pub struct Application<'a> {
    event_loop: EventLoop<()>,
    window: Window,
    input_manager: WinitInputHelper,
    renderer: Renderer<'a>,
}

impl Application<'static> {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        let window_settings = WindowSettings::default();
        let window = Window::new(window_settings, &event_loop);

        let input_manager = WinitInputHelper::new();
        let renderer = Renderer::new(&window.display);

        Self {
            event_loop,
            window,
            input_manager,
            renderer,
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            if self.input_manager.update(&event) {
                if self.input_manager.key_pressed(VirtualKeyCode::Escape) {
                    *control_flow = ControlFlow::Exit;
                }
            }

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                Event::MainEventsCleared => {
                    self.renderer.draw(&self.window.display);
                }
                _ => {}
            }
        });
    }
}
