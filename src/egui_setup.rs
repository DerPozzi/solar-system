use std::{collections::HashSet, time::Instant};

use anyhow::Result;

use egui::{TextWrapMode, ViewportId};
use glium::{
    Surface,
    glutin::surface::WindowSurface,
    winit::{
        self,
        application::ApplicationHandler,
        event::{DeviceEvent, DeviceId, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        keyboard::PhysicalKey,
        window::{Window, WindowId},
    },
};
use log::debug;

use crate::{camera::Camera, skybox::Skybox};

pub struct GliumAttributes {
    pub window: Window,
    pub display: glium::Display<WindowSurface>,
}

pub struct App {
    egui_glium: egui_glium::EguiGlium,
    pub glium_attributes: GliumAttributes,
    delta_time: f32,
    last_frame: std::time::Instant,
    pub skybox: Skybox,
    camera: Camera,
    keys_pressed: HashSet<PhysicalKey>,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>, glium_attributes: GliumAttributes) -> Self {
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
    fn redraw(&mut self) -> Result<()> {
        let dist = self.camera.get_dist();
        let fov = self.camera.get_fov();

        let mut frame = self.glium_attributes.display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        let (width, height) = frame.get_dimensions();

        let aspect_ratio = width as f32 / height as f32;

        let view = self.camera.get_view_matrix();
        let perspective = glam::Mat4::perspective_rh(fov, aspect_ratio, 0.1, dist);
        let skybox_uniforms = uniform! {
            view: view.to_cols_array_2d(),
            perspective: perspective.to_cols_array_2d(),
            skybox: self.skybox.cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
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
            &self.skybox.vertex_buffer,
            &self.skybox.index_buffer,
            &self.skybox.cubemap_program,
            &skybox_uniforms,
            &params,
        )?;

        self.egui_glium
            .paint(&self.glium_attributes.display, &mut frame);
        frame.finish()?;

        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        debug!("Resumed");
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
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

                let _ = self.redraw();
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
        _device_id: DeviceId,
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
