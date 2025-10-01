use std::{collections::HashSet, time::Instant};

use anyhow::Result;
use egui::{TextWrapMode, ViewportId};
use glium::{
    backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, winit::{
        self,
        application::ApplicationHandler,
        event::{DeviceEvent, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        keyboard::PhysicalKey,
        window::{CursorGrabMode, Window},
    }, Surface
};
use log::{debug, info};

use crate::{camera::Camera, skybox::Skybox};

mod camera;
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

fn upload_sky_box(app: &App) {
    let dest_rect1 = glium::BlitTarget {
        left: 0,
        bottom: 0,
        width: 512,
        height: 512,
    };

    let framebuffer1 = glium::framebuffer::SimpleFrameBuffer::new(
        &app.glium_attributes.display,
        app.skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::PositiveX),
    )
    .unwrap();
    let framebuffer2 = glium::framebuffer::SimpleFrameBuffer::new(
        &app.glium_attributes.display,
        app.skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::NegativeX),
    )
    .unwrap();
    let framebuffer3 = glium::framebuffer::SimpleFrameBuffer::new(
        &app.glium_attributes.display,
        app.skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::PositiveY),
    )
    .unwrap();
    let framebuffer4 = glium::framebuffer::SimpleFrameBuffer::new(
        &app.glium_attributes.display,
        app.skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::NegativeY),
    )
    .unwrap();
    let framebuffer5 = glium::framebuffer::SimpleFrameBuffer::new(
        &app.glium_attributes.display,
        app.skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::PositiveZ),
    )
    .unwrap();
    let framebuffer6 = glium::framebuffer::SimpleFrameBuffer::new(
        &app.glium_attributes.display,
        app.skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::NegativeZ),
    )
    .unwrap();

    let tex_posx = &app.skybox.textures[0];
    let tex_negx = &app.skybox.textures[1];
    let tex_posy = &app.skybox.textures[2];
    let tex_negy = &app.skybox.textures[3];
    let tex_posz = &app.skybox.textures[4];
    let tex_negz = &app.skybox.textures[5];

    tex_posx.as_surface().blit_whole_color_to(
        &framebuffer1,
        &dest_rect1,
        glium::uniforms::MagnifySamplerFilter::Linear,
    );
    tex_negx.as_surface().blit_whole_color_to(
        &framebuffer2,
        &dest_rect1,
        glium::uniforms::MagnifySamplerFilter::Linear,
    );
    tex_negy.as_surface().blit_whole_color_to(
        &framebuffer3,
        &dest_rect1,
        glium::uniforms::MagnifySamplerFilter::Linear,
    );
    tex_posy.as_surface().blit_whole_color_to(
        &framebuffer4,
        &dest_rect1,
        glium::uniforms::MagnifySamplerFilter::Linear,
    );
    tex_posz.as_surface().blit_whole_color_to(
        &framebuffer5,
        &dest_rect1,
        glium::uniforms::MagnifySamplerFilter::Linear,
    );
    tex_negz.as_surface().blit_whole_color_to(
        &framebuffer6,
        &dest_rect1,
        glium::uniforms::MagnifySamplerFilter::Linear,
    );
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

fn redraw(app: &mut App) -> Result<()> {
    let dist = app.camera.get_dist();
    let fov = app.camera.get_fov();

    upload_sky_box(app);

    let mut frame = app.glium_attributes.display.draw();
    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    let (width, height) = frame.get_dimensions();

    let aspect_ratio = width as f32 / height as f32;

    let view = app.camera.get_view_matrix();
    let perspective = glam::Mat4::perspective_rh(fov, aspect_ratio, 0.1, dist);
    let skybox_uniforms = uniform! {
        view: view.to_cols_array_2d(),
        perspective: perspective.to_cols_array_2d(),
        skybox: app.skybox.cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise, // Remove this comment to enable backface culling - Not drawing the back faces of the triangles
        ..Default::default()
    };

    frame.draw(
        &app.skybox.vertex_buffer,
        &app.skybox.index_buffer,
        &app.skybox.cubemap_program,
        &skybox_uniforms,
        &params,
    )?;

    app.egui_glium
        .paint(&app.glium_attributes.display, &mut frame);
    frame.finish()?;

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
    skybox: Skybox,
    camera: Camera,
    keys_pressed: HashSet<PhysicalKey>,
}

impl App {
    fn new(event_loop: &EventLoop<()>, glium_attributes: GliumAttributes) -> Self {
        let dist = 10_000.0;
        let fov = 90.0;
        let camera = Camera::new(fov, dist);
        let skybox = Skybox::init(&glium_attributes.display, dist);
        let keys_pressed: HashSet<PhysicalKey> = std::collections::HashSet::new();
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
            skybox,
            camera,
            keys_pressed,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        debug!("Resumed");
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
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

                let _ = redraw(self);
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
            DeviceEvent::MouseMotion { delta } => {
                // if !self.ui_visible {
                let mouse_sensitivity = 0.001;
                self.camera.add_to_yaw_pitch(
                    -delta.0 as f32 * mouse_sensitivity, // yaw
                    -delta.1 as f32 * mouse_sensitivity, // pitch
                );

                self.camera.apply_yaw_pitch();
                // }
            }
            DeviceEvent::MouseWheel { delta } => {
                let _ = delta;
                // println!("Mouse wheel: delta={:?}", delta);
            }
            _ => (),
        }

        self.glium_attributes.window.request_redraw();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;

        self.delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = std::time::Instant::now();

        self.camera
            .update_camera(&self.keys_pressed, self.delta_time);
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
