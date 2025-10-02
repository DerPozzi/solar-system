use anyhow::Result;
use glium::{
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    winit::{
        self,
        event_loop::EventLoop,
        window::{CursorGrabMode, Window},
    },
};
use log::info;

use crate::{
    egui_setup::{App, GliumAttributes},
    skybox::upload_sky_box,
};

mod camera;
mod egui_setup;
mod skybox;

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

    window
        .set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))?;

    window.set_cursor_visible(false);

    let mut app = App::new(&event_loop, GliumAttributes { window, display });

    upload_sky_box(&app);

    info!("Entering main loop...");
    event_loop.run_app(&mut app)?;
    Ok(())
}
