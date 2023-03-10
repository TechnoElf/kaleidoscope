use std::time::Instant;

use glium::{Display, Surface};
use glium::glutin::ContextBuilder;
use glium::glutin::event_loop::{EventLoop, ControlFlow};
use glium::glutin::window::WindowBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};

use imgui::*;

use imgui_winit_support::{WinitPlatform, HiDpiMode};
use imgui_glium_renderer::Renderer;

pub struct UiContext<F: FnMut(&Ui, &mut T) + 'static, T: 'static> {
    event_loop: EventLoop<()>,
    display: Display,
    imgui: Context,
    renderer: Renderer,
    platform: WinitPlatform,
    last_frame: Instant,
    data: T,
    main_screen: F
}

impl<F: FnMut(&Ui, &mut T) + 'static, T: 'static> UiContext<F, T> {
    pub fn new(main_screen: F, data: T) -> Self {
        let event_loop = EventLoop::new();
        let glium_context = ContextBuilder::new().with_vsync(true);
        let window_builder = WindowBuilder::new()
            .with_title("kaleidoscope".to_owned())
            .with_inner_size(LogicalSize::new(1024.0, 758.0));
        let display = Display::new(window_builder, glium_context, &event_loop).unwrap();

        let mut imgui = Context::create();

        let mut platform = WinitPlatform::init(&mut imgui);

        platform.attach_window(imgui.io_mut(), display.gl_window().window(), HiDpiMode::Default);

        let renderer = Renderer::init(&mut imgui, &display).unwrap();

        Self {
            event_loop,
            display,
            imgui,
            renderer,
            platform,
            last_frame: Instant::now(),
            data,
            main_screen
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                self.imgui.io_mut().update_delta_time(now - self.last_frame);
                self.last_frame = now;
            },
            Event::MainEventsCleared => {
                self.platform.prepare_frame(self.imgui.io_mut(), self.display.gl_window().window()).unwrap();
                self.display.gl_window().window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                let ui = self.imgui.frame();

                {
                    let size = self.display.gl_window().window().inner_size();
                    let token = ui.window("Main")
                        .position([0.0, 0.0], Condition::Always)
                        .size([size.width as f32, size.height as f32], Condition::Always)
                        .flags(
                            WindowFlags::MENU_BAR | WindowFlags::NO_TITLE_BAR | WindowFlags::NO_COLLAPSE |
                            WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE | WindowFlags::NO_NAV_FOCUS |
                            WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS | WindowFlags::NO_SCROLLBAR | WindowFlags::NO_SCROLL_WITH_MOUSE
                        )
                        .begin();
                    if let Some(_) = token {
                        (self.main_screen)(&ui, &mut self.data);
                    }
                }

                let mut target = self.display.draw();
                target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
                self.platform.prepare_render(&ui, self.display.gl_window().window());
                let draw_data = self.imgui.render();
                self.renderer.render(&mut target, draw_data).unwrap();
                target.finish().unwrap();
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            event => self.platform.handle_event(self.imgui.io_mut(), self.display.gl_window().window(), &event)
        });
    }
}
