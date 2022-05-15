use glium::glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

use crate::window::{Window, WindowSettings};

pub struct Application {
    event_loop: EventLoop<()>,
    window: Window,
    input_manager: WinitInputHelper,
}

impl Application {
    pub fn new() -> Self {
        let mut event_loop = EventLoop::new();

        let window_settings = WindowSettings::default();
        let window = Window::new(window_settings, &event_loop);

        let input_manager = WinitInputHelper::new();

        Self {
            event_loop,
            window,
            input_manager,
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
                Event::MainEventsCleared => {}
                _ => {}
            }
        });
    }
}
