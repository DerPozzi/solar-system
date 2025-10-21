use std::path::PathBuf;

use anyhow::Result;
use glium::{Display, glutin::surface::WindowSurface};
use gltf::{buffer::Data, mesh::util::ReadTexCoords};
use log::info;

pub struct CelestialBody {
    name: String,
    radius: f32,
    distance_from_sun: f32,
    orbital_period: f32,
    rotation_period: f32,
    texture_path: String,
}

#[derive(Debug, Clone, Copy)]
struct Vertex {
    pos: [f32; 3],
    uv: [f32; 3],
    tex: [f32; 2],
}
implement_vertex!(Vertex, pos, uv, tex);

pub fn load_cb_from_gltf(names: Vec<&str>, display: &Display<WindowSurface>) -> Result<()> {
    info!("Loading models from file:");
    for name in names.iter() {
        println!("|\t{}", name);
        let cwd = std::env::current_dir()?;
        let path = cwd.join(PathBuf::from(format!(
            "assets/models/{}/scene.gltf",
            name.to_lowercase()
        )));

        // let (document, buffers, images) = gltf::import(path)?;
        let (document, buffers, images) = match gltf::import(&path) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("❌ Fehler beim Laden von {:?}: {}", path, e);
                break;
            }
        };
        let _cb = CelestialBody {
            name: name.to_string(),
            radius: 1.0,
            distance_from_sun: 1.0,
            orbital_period: 1.0,
            rotation_period: 1.0,
            texture_path: String::new(),
        };

        // get_sphere(&document, &buffers, &images, display)?;
    }
    Ok(())
}

fn get_sphere(
    doc: &gltf::Document,
    buffers: &Vec<Data>,
    images: &Vec<gltf::image::Data>,
    display: &Display<WindowSurface>,
) -> Result<()> {
    let mut pos: Vec<[f32; 3]> = Vec::new();
    if let Some(node) = doc
        .nodes()
        .find(|n| n.name() == Some("Sphere_Material.002_0"))
    {
        if let Some(mesh) = node.mesh() {
            println!("Mesh gefunden: {:?}", mesh.name());

            for prim in mesh.primitives() {
                let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));

                // Vertexpositionen
                if let Some(positions) = reader.read_positions() {
                    // for p in positions {
                    //     // p: [f32; 3]
                    // }
                    pos = positions.collect();
                }

                // Indizes
                if let Some(indices) = reader.read_indices() {
                    for i in indices.into_u32() {
                        // i: u32
                    }
                }

                // UV-Koordinaten
                if let Some(uvs) = reader.read_tex_coords(0) {
                    match uvs {
                        ReadTexCoords::F32(iter) => {
                            for uv in iter {
                                // uv: [f32; 2]
                            }
                        }
                        _ => {}
                    }
                }

                // Material & Texturen (optional)
                if let Some(material) = prim.material().pbr_specular_glossiness() {
                    let tex = material.diffuse_texture().unwrap().texture();
                    println!("Textur gefunden: {:?}", tex.source().index());
                    // Du kannst `images[tex.source().index()]` verwenden,
                    // wenn du die Textur weiterverarbeiten willst.
                    println!(
                        "Texturgröße: {}x{}",
                        images[tex.source().index()].width,
                        images[tex.source().index()].height
                    );
                }
            }
        }
    }

    let vb = glium::VertexBuffer::new(
        display,
        &pos.iter()
            .map(|&p| Vertex {
                pos: p,
                uv: [0.0, 0.0, 0.0],
                tex: [0.0, 0.0],
            })
            .collect::<Vec<Vertex>>(),
    )?;
    info!("Vertex Buffer erstellt mit {} Vertices", vb.len());
    Ok(())
}

fn print_node(node: &gltf::Node, depth: usize) -> Result<()> {
    let indent = "  ".repeat(depth);
    println!(
        "{}Node: {} - {}",
        indent,
        node.name().unwrap_or("Unnamed"),
        node.index()
    );
    if let Some(mesh) = node.mesh() {
        println!("{} Mesh: {}", indent, mesh.name().unwrap_or("Unnamed"));
        for primitive in mesh.primitives() {
            println!("{}  Primitive: {:?}", indent, primitive.mode());
        }
    }
    for child in node.children() {
        print_node(&child, depth + 1)?;
    }
    Ok(())
}
