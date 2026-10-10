mod line;
mod obj;

use obj::{Obj, load_obj};
use raylib::prelude::*;
use std::{error::Error, fs, io, path::Path};

const WIDTH: i32 = 2400;
const HEIGHT: i32 = 1920;
const SAMPLES: i32 = 2;

fn render_model(model: &Obj) -> Image {
    let yaw = 32.0_f32.to_radians();
    let pitch = 22.0_f32.to_radians();
    let vertices: Vec<Vector2> = model
        .vertices
        .iter()
        .map(|v| {
            let x = yaw.cos() * v.x - yaw.sin() * v.z;
            let depth = yaw.sin() * v.x + yaw.cos() * v.z;
            let y = pitch.cos() * v.y - pitch.sin() * depth;
            Vector2::new(x, y)
        })
        .collect();

    let min_x = vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min);
    let max_x = vertices
        .iter()
        .map(|v| v.x)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
    let max_y = vertices
        .iter()
        .map(|v| v.y)
        .fold(f32::NEG_INFINITY, f32::max);
    let margin = 140.0;
    let scale = ((WIDTH as f32 - 2.0 * margin) / (max_x - min_x).max(0.001))
        .min((HEIGHT as f32 - 2.0 * margin) / (max_y - min_y).max(0.001));
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    let screen_vertices: Vec<Vector2> = vertices
        .iter()
        .map(|v| {
            Vector2::new(
                ((v.x - center_x) * scale + WIDTH as f32 / 2.0) * SAMPLES as f32,
                (HEIGHT as f32 / 2.0 - (v.y - center_y) * scale) * SAMPLES as f32,
            )
        })
        .collect();

    let mut image = Image::gen_image_color(WIDTH * SAMPLES, HEIGHT * SAMPLES, Color::BLACK);
    for face in model.indices.chunks_exact(3) {
        let a = screen_vertices[face[0]];
        let b = screen_vertices[face[1]];
        let c = screen_vertices[face[2]];
        line::triangle(&mut image, a, b, c);
    }
    image.resize(WIDTH, HEIGHT);
    image
}

fn save_image(image: &Image, path: &Path) -> Result<(), Box<dyn Error>> {
    let filename = path
        .to_str()
        .ok_or_else(|| io::Error::other("La ruta de la imagen no es valida"))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?;
    image.export_image(filename);
    if fs::metadata(path)?.len() == 0 {
        return Err(io::Error::other("No se pudo guardar la imagen PNG").into());
    }
    println!("Imagen guardada: {}", path.display());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = root.join("assets/nave_kylo2.obj");
    let model = load_obj(
        path.to_str()
            .ok_or_else(|| io::Error::other("Ruta OBJ invalida"))?,
    )?;
    println!(
        "Vertices: {} | Indices: {} | Triangulos: {}",
        model.vertices.len(),
        model.indices.len(),
        model.indices.len() / 3
    );
    let image = render_model(&model);
    let output = root.join("docs/captura.png");
    save_image(&image, &output)?;

    let (mut window, thread) = raylib::init()
        .size(1000, 800)
        .title("Proyecto 1 - Parte 1 - Nave de Kylo Ren")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    window.set_target_fps(60);
    let texture = window.load_texture_from_image(&thread, &image)?;
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_S) {
            if let Err(error) = save_image(&image, &output) {
                eprintln!("Error al guardar: {error}");
            }
        }
        let width = window.get_screen_width() as f32;
        let height = window.get_screen_height() as f32;
        let size = (width / WIDTH as f32).min(height / HEIGHT as f32);
        let destination = Rectangle::new(
            (width - WIDTH as f32 * size) / 2.0,
            (height - HEIGHT as f32 * size) / 2.0,
            WIDTH as f32 * size,
            HEIGHT as f32 * size,
        );
        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32),
            destination,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
    Ok(())
}
