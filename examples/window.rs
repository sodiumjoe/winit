extern crate winit;

fn main() {
    let mut events_loop = winit::EventsLoop::new();

    let _window = winit::WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&events_loop)
        .unwrap();

    events_loop.run_forever(|event| {
        match event {
            // winit::Event::WindowEvent { event: winit::WindowEvent::Resized(x, y), .. } => {
            //     println!("resized: {},{}", x, y);
            //     winit::ControlFlow::Continue
            // }
            // winit::Event::WindowEvent { event: winit::WindowEvent::MouseInput { state, .. }, .. } => {
            //     println!("mouse input: {:?}", state);
            //     winit::ControlFlow::Continue
            // }
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
        }
    });
}
