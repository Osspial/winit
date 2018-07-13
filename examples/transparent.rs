extern crate winit;

fn main() {
    let mut events_loop = winit::EventLoop::new();

    let window = winit::WindowBuilder::new().with_decorations(false)
                                                 .with_transparency(true)
                                                 .build(&events_loop).unwrap();

    window.set_title("A fantastic window!");

    events_loop.run_forever(move |event, _: &winit::EventLoop| {
        println!("{:?}", event);

        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    });
}
