use anyhow::Result;
use std::{fs::File, io::BufReader};

use glium::{Display, Surface, Texture2d, glutin::surface::WindowSurface, texture::Cubemap};
use log::{ info};

use crate::egui_setup::App;

pub struct Skybox {
    pub textures: Vec<Texture2d>,
    pub vertex_buffer: glium::VertexBuffer<SkyboxVertex>,
    pub cubemap_program: glium::Program,
    pub cubemap: Cubemap,
    pub index_buffer: glium::IndexBuffer<u16>,
}

impl Skybox {
    pub fn init(display: &Display<WindowSurface>, cam_dist: f32) -> Self {
        info!("Initializing skybox...");
        Skybox {
            textures: load_skybox_textures(display),
            vertex_buffer: create_skybox_vb(&display, cam_dist).unwrap(),
            cubemap_program: glium::Program::from_source(
                display,
                include_str!("../assets/shaders/skybox.vert"),
                include_str!("../assets/shaders/skybox.frag"),
                None,
            )
            .unwrap(),
            cubemap: Cubemap::empty(display, 512).unwrap(), // Placeholder, will be filled later
            index_buffer: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &[
                    // Front
                    0u16, 2, 1, 0, 3, 2, // Right
                    4, 6, 5, 4, 7, 6, // Back
                    8, 10, 9, 8, 11, 10, // Left
                    12, 14, 13, 12, 15, 14, // Bottom
                    16, 18, 17, 16, 19, 18, // Top
                    20, 22, 21, 20, 23, 22,
                ],
            )
            .unwrap(),
        }
    }
}

pub fn upload_sky_box(app: &App) {
    let dest_rect1 = glium::BlitTarget {
        left: 0,
        bottom: 0,
        width: 512,
        height: 512,
    };

    let display = &app.glium_attributes.display;
    let skybox = &app.skybox;

    let framebuffer1 = glium::framebuffer::SimpleFrameBuffer::new(
        display,
        skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::PositiveX),
    )
    .unwrap();
    let framebuffer2 = glium::framebuffer::SimpleFrameBuffer::new(
        display,
        skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::NegativeX),
    )
    .unwrap();
    let framebuffer3 = glium::framebuffer::SimpleFrameBuffer::new(
        display,
        skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::PositiveY),
    )
    .unwrap();
    let framebuffer4 = glium::framebuffer::SimpleFrameBuffer::new(
        display,
        skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::NegativeY),
    )
    .unwrap();
    let framebuffer5 = glium::framebuffer::SimpleFrameBuffer::new(
        display,
        skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::PositiveZ),
    )
    .unwrap();
    let framebuffer6 = glium::framebuffer::SimpleFrameBuffer::new(
        display,
        skybox
            .cubemap
            .main_level()
            .image(glium::texture::CubeLayer::NegativeZ),
    )
    .unwrap();

    let tex_posx = &skybox.textures[0];
    let tex_negx = &skybox.textures[1];
    let tex_posy = &skybox.textures[2];
    let tex_negy = &skybox.textures[3];
    let tex_posz = &skybox.textures[4];
    let tex_negz = &skybox.textures[5];

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

fn load_skybox_textures(display: &Display<WindowSurface>) -> Vec<Texture2d> {
    info!("Loading skybox textures...");
    let base_path = "assets/skybox/";
    let face_filenames = [
        "right.png",  // Positive X
        "left.png",   // Negative X
        "top.png",    // Positive Y
        "bottom.png", // Negative Y
        "front.png",  // Positive Z
        "back.png",   // Negative Z
    ];

    let time_start = std::time::Instant::now();

    let images = face_filenames
        .iter()
        .map(|filename| {
            let path = format!("{}{}", base_path, filename);
            let reader = BufReader::new(File::open(&path).unwrap());
            let image = image::load(reader, image::ImageFormat::Png)
                .unwrap()
                .to_rgba8();
            let image_dimensions = image.dimensions();
            let image =
                glium::texture::RawImage2d::from_raw_rgba_reversed(&image, image_dimensions);
            glium::Texture2d::new(display, image).unwrap()
        })
        .collect();

    info!("Loaded skybox textures in {:.2?}", time_start.elapsed());
    images
}

#[derive(Copy, Clone)]
pub struct SkyboxVertex {
    position: [f32; 3],
}

implement_vertex!(SkyboxVertex, position);

fn create_skybox_vb(
    display: &Display<WindowSurface>,
    cam_dist: f32,
) -> Result<glium::VertexBuffer<SkyboxVertex>> {
    let side2 = cam_dist / 2.0;
    let skybox_vertices = [
        // Front
        SkyboxVertex {
            position: [-side2, -side2, side2],
        },
        SkyboxVertex {
            position: [side2, -side2, side2],
        },
        SkyboxVertex {
            position: [side2, side2, side2],
        },
        SkyboxVertex {
            position: [-side2, side2, side2],
        },
        // Right
        SkyboxVertex {
            position: [side2, -side2, side2],
        },
        SkyboxVertex {
            position: [side2, -side2, -side2],
        },
        SkyboxVertex {
            position: [side2, side2, -side2],
        },
        SkyboxVertex {
            position: [side2, side2, side2],
        },
        // Back
        SkyboxVertex {
            position: [-side2, -side2, -side2],
        },
        SkyboxVertex {
            position: [-side2, side2, -side2],
        },
        SkyboxVertex {
            position: [side2, side2, -side2],
        },
        SkyboxVertex {
            position: [side2, -side2, -side2],
        },
        // Left
        SkyboxVertex {
            position: [-side2, -side2, side2],
        },
        SkyboxVertex {
            position: [-side2, side2, side2],
        },
        SkyboxVertex {
            position: [-side2, side2, -side2],
        },
        SkyboxVertex {
            position: [-side2, -side2, -side2],
        },
        // Bottom
        SkyboxVertex {
            position: [-side2, -side2, side2],
        },
        SkyboxVertex {
            position: [-side2, -side2, -side2],
        },
        SkyboxVertex {
            position: [side2, -side2, -side2],
        },
        SkyboxVertex {
            position: [side2, -side2, side2],
        },
        // Top
        SkyboxVertex {
            position: [-side2, side2, side2],
        },
        SkyboxVertex {
            position: [side2, side2, side2],
        },
        SkyboxVertex {
            position: [side2, side2, -side2],
        },
        SkyboxVertex {
            position: [-side2, side2, -side2],
        },
    ];
    let test = glium::VertexBuffer::new(display, &skybox_vertices)?;
    Ok(test)
}
