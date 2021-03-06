// This example is used by developers to test various window functions.

use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(LogicalSize::from((100, 100)))
        .build(&event_loop)
        .unwrap();

    eprintln!("debugging keys:");
    eprintln!("  (E) Enter exclusive fullscreen");
    eprintln!("  (F) Toggle borderless fullscreen");
    #[cfg(waiting_for_set_minimized)]
    eprintln!("  (M) Toggle minimized");
    eprintln!("  (Q) Quit event loop");
    eprintln!("  (V) Toggle visibility");
    eprintln!("  (X) Toggle maximized");

    #[cfg(waiting_for_set_minimized)]
    let mut minimized = false;
    let mut maximized = false;
    let mut visible = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Pressed,
                        ..
                    }),
                ..
            } => match key {
                #[cfg(waiting_for_set_minimized)]
                VirtualKeyCode::M => {
                    if minimized {
                        minimized = !minimized;
                        window.set_minimized(minimized);
                    }
                }
                VirtualKeyCode::V => {
                    if !visible {
                        visible = !visible;
                        window.set_visible(visible);
                    }
                }
                _ => (),
            },
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input {
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state: ElementState::Pressed,
                    ..
                } => match key {
                    VirtualKeyCode::E => {
                        fn area(size: PhysicalSize) -> f64 {
                            size.width * size.height
                        }

                        let monitor = window.current_monitor();
                        if let Some(mode) = monitor.video_modes().max_by(|a, b| {
                            area(a.size())
                                .partial_cmp(&area(b.size()))
                                .expect("NaN in video mode size")
                        }) {
                            window.set_fullscreen(Some(Fullscreen::Exclusive(mode)));
                        } else {
                            eprintln!("no video modes available");
                        }
                    }
                    VirtualKeyCode::F => {
                        if window.fullscreen().is_some() {
                            window.set_fullscreen(None);
                        } else {
                            let monitor = window.current_monitor();
                            window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
                        }
                    }
                    #[cfg(waiting_for_set_minimized)]
                    VirtualKeyCode::M => {
                        minimized = !minimized;
                        window.set_minimized(minimized);
                    }
                    VirtualKeyCode::Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    VirtualKeyCode::V => {
                        visible = !visible;
                        window.set_visible(visible);
                    }
                    VirtualKeyCode::X => {
                        maximized = !maximized;
                        window.set_maximized(maximized);
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
