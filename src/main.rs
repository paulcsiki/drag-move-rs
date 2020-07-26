use winit::window::WindowBuilder;
use glium::{Display};
use glium::glutin;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use winit::dpi::PhysicalPosition;
use winit::event::{Event, WindowEvent, MouseButton, ElementState};
use imgui::{Context, Ui};
use imgui_winit_support::{WinitPlatform, HiDpiMode};

fn main() {
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_title("drag")
        .with_always_on_top(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(300f64, 300f64));
    let display = Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
    }

    let mut mouse_down = false;
    let mut last_pos: Option<PhysicalPosition<f64>> = None;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        Event::WindowEvent {
            event: WindowEvent::CursorMoved {
                position,
                ..
            },
            ..
        } => {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            if mouse_down {
                if last_pos.is_some() {
                    let previous_pos = last_pos.unwrap();
                    let delta_x = previous_pos.x - position.x;
                    let delta_y = previous_pos.y - position.y;
                    window.set_outer_position(PhysicalPosition::new(position.x + delta_x, position.y + delta_y));
                }
                last_pos = Some(position);
            }
        }
        Event::WindowEvent {
            event: WindowEvent::MouseInput{
                state,
                button,
                ..
            },
            ..
        } => {
            mouse_down = button == MouseButton::Left && state == ElementState::Pressed;
            if !mouse_down {
                last_pos = None;
            }
        }
        _ => {}
    });
}
