use std::time::Instant;

use anyhow::Result;
use egui::{TextWrapMode, ViewportId};
use glium::{
    Surface,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    winit::{
        self,
        application::ApplicationHandler,
        event::{DeviceEvent, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        window::Window,
    },
};
use log::{debug, info};

#[macro_use]
extern crate glium;

fn create_display(
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, glium::Display<WindowSurface>) {
    SimpleWindowBuilder::new()
        .set_window_builder(Window::default_attributes())
        .with_title("Solar System")
        .build(event_loop)
}

fn main() -> Result<()> {
    colog::init();
    info!("Starting application...");
    let event_loop = EventLoop::builder().build()?;
    let (window, display) = create_display(&event_loop);

    let mut app = App::new(
        &event_loop,
        GliumAttributes {
            window: window,
            display: display,
        },
    );

    info!("Entering main loop...");
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct GliumAttributes {
    window: Window,
    display: glium::Display<WindowSurface>,
}

struct App {
    egui_glium: egui_glium::EguiGlium,
    glium_attributes: GliumAttributes,
    delta_time: f32,
    last_frame: std::time::Instant,
}

impl App {
    fn new(event_loop: &EventLoop<()>, glium_attributes: GliumAttributes) -> Self {
        let egui_glium = egui_glium::EguiGlium::new(
            ViewportId::ROOT,
            &glium_attributes.display,
            &glium_attributes.window,
            event_loop,
        );
        Self {
            egui_glium,
            glium_attributes,
            delta_time: 0.0,
            last_frame: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        debug!("Resumed");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match &event {
            WindowEvent::Resized(physical_size) => {
                self.glium_attributes
                    .display
                    .resize((*physical_size).into());
            }

            WindowEvent::RedrawRequested => {
                let egui_glium = &mut self.egui_glium;
                let window = &self.glium_attributes.window;
                egui_glium.run(window, |egui_ctx| {
                    egui::Area::new("fps_hud".into())
                        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
                        .order(egui::Order::Foreground)
                        .show(egui_ctx, |ui| {
                            ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                            egui::Frame::NONE
                                .fill(egui::Color32::TRANSPARENT)
                                .corner_radius(6.0)
                                .inner_margin(egui::Margin::symmetric(8, 4))
                                .show(ui, |ui| {
                                    ui.monospace(format!("{:.0}", 1.0 / self.delta_time));
                                });
                        });
                });

                let mut target = self.glium_attributes.display.draw();
                target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);
                egui_glium.paint(&self.glium_attributes.display, &mut target);
                target.finish().unwrap();
            }

            _ => {}
        }
    }
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        let _ = (event_loop, cause);
        if let winit::event::StartCause::ResumeTimeReached { .. } = cause {
            self.glium_attributes.window.request_redraw();
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
        let _ = (event_loop, event);
        println!("User event received {:?}", event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;

        self.delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = std::time::Instant::now();

        // self.cam.update_camera(&self.keys_pressed, self.delta_time);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }
}
