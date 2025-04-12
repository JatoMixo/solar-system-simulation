use winit::{
    application::ApplicationHandler, event_loop::EventLoop, keyboard::NamedKey, window::{Window, WindowAttributes}, event::WindowEvent
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(WindowAttributes::default()).expect("Error creating window"));
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        
        // Close when pressing Escape
        if let WindowEvent::KeyboardInput { event, .. } = &event {
            if let winit::keyboard::Key::Named(NamedKey::Escape) = event.logical_key {
                event_loop.exit();
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("Could't build the event loop");
    
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("The app crashed for some reason");
}
