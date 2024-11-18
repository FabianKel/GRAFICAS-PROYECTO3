use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, Mat4, look_at, perspective, rotate_y};
use utils::frustum_culling;
use std::time::{Duration, Instant};
use std::f32::consts::PI;

mod utils;
mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod spaceship;


use spaceship::Spaceship;
use framebuffer::Framebuffer;
use crate::fragment::Fragment;
use crate::color::Color;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use camera::Camera;
use shaders::{ring_shader, rocky_planet_shader, gas_giant_shader, gas_giant_shader2, volcanic_planet_shader, icy_planet_shader, desert_planet_shader, water_planet_shader, moon_shader, vertex_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType};

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
    pub noise: FastNoiseLite,
}

struct Planet {
    position: Vec3,
    scale: f32,
    rotation: Vec3,
    shader: fn(&Fragment, &Uniforms) -> Color,
    vertices: Vec<Vertex>,
}


fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation_angle: f32) -> Mat4 {
    Mat4::new_translation(&translation)
        * Mat4::from_axis_angle(&Vec3::y_axis(), rotation_angle)
        * Mat4::new_scaling(scale)
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    perspective(fov, aspect_ratio, 0.1, 1000.0)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}




fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);
    let start_time = Instant::now();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new("Proyecto 3 - GPC", window_width, window_height, WindowOptions::default())
        .unwrap();

    window.set_position(500, 500);
    framebuffer.set_background_color(0x333355);



    let obj = Obj::load("assets/models/sphere2.obj").expect("Error al cargar el modelo");
    let ring_obj = Obj::load("assets/models/ring1.obj").expect("Error al cargar el modelo del aro");

    
    let planets = vec![
    //planeta 1
    Planet {
        position: Vec3::new(2.0, 0.0, 0.0),
        scale: 0.5,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: desert_planet_shader,
        vertices: obj.get_vertex_array(),
    },
    //Planeta 2 con Aro
    Planet {
        position: Vec3::new(4.0, 0.0, 0.0),
        scale: 0.8,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: ring_shader,
        vertices: ring_obj.get_vertex_array(),
    },
    Planet {
        position: Vec3::new(4.0, 0.0, 0.0),
        scale: 0.6,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: gas_giant_shader2,
        vertices: obj.get_vertex_array(),
    },
    //Planeta 3
    Planet {
        position: Vec3::new(6.0, 0.0, 0.0),
        scale: 0.6,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: volcanic_planet_shader,
        vertices: obj.get_vertex_array(),
    },
    //Planeta 4
    Planet {
        position: Vec3::new(8.0, 0.0, 0.0),
        scale: 0.6,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: gas_giant_shader,
        vertices: obj.get_vertex_array(),
    },    
    //Planeta 5
    Planet {
        position: Vec3::new(10.0, 0.0, 0.0),
        scale: 0.6,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: water_planet_shader,
        vertices: obj.get_vertex_array(),
    },
    //Planeta 6
    Planet {
        position: Vec3::new(12.0, 0.0, 0.0),
        scale: 0.6,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: icy_planet_shader,
        vertices: obj.get_vertex_array(),
    },
    //Planeta 6
    Planet {
        position: Vec3::new(14.0, 0.0, 0.0),
        scale: 0.6,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: rocky_planet_shader,
        vertices: obj.get_vertex_array(),
    },
    //Luna del planeta 1
    Planet {
        position: Vec3::new(2.0, 0.0, 2.0),
        scale: 0.2,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        shader: moon_shader,
        vertices: obj.get_vertex_array(),
    },
];


    let spaceship_obj = Obj::load("assets/models/mini_espacioship.obj").expect("Error al cargar el modelo de la nave espacial");

    let mut spaceship = Spaceship::new(
        Vec3::new(14.0, 0.0, 18.0), // Posición inicial
        Vec3::new(0.0, 0.0, 0.0), // Rotación inicial
        1.0,                      // Escala
        spaceship_obj.get_vertex_array(), 
        desert_planet_shader,
    );
    let mut camera = Camera::new(
        Vec3::new(0.0, 10.0, 20.0),
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    

    
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
    
        handle_input(&window, &mut spaceship, &mut camera);
        update_camera(&mut camera, &spaceship);
    
        framebuffer.clear();
    
        let time_elapsed = start_time.elapsed().as_secs_f32();
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32 *0.5, window_height as f32 *0.5);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    
        for planet in &planets {
            // **Aplicar frustum culling**: verificar si el planeta está dentro del frustum
            if !frustum_culling(
                &planet.position,
                planet.scale,
                &view_matrix,
                &projection_matrix,
            ) {
                continue; // Saltar este planeta si está fuera del frustum
            }
    
            // Calcular matriz de modelo del planeta
            let model_matrix = create_model_matrix(planet.position, planet.scale, time_elapsed);
    
            // Las matrices de vista, proyección y viewport son constantes para todos los planetas
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time: time_elapsed,
                noise: create_noise(),
            };
    
            // Transformar los vértices del planeta en función de sus propias matrices
            let transformed_vertices = planet.vertices.iter()
                .map(|vertex| vertex_shader(vertex, &uniforms))
                .collect::<Vec<_>>();
    
            // Dividir en triángulos
            let triangles = transformed_vertices.chunks(3)
                .filter(|tri| tri.len() == 3)
                .map(|tri| [tri[0].clone(), tri[1].clone(), tri[2].clone()])
                .collect::<Vec<_>>();
    
            let mut fragments = Vec::new();
    
            // Generar fragmentos para rasterizar
            for tri in &triangles {
                fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
            }
    
            // Dibujar los fragmentos en el framebuffer
            for fragment in fragments {
                let (x, y) = (fragment.position.x as usize, fragment.position.y as usize);
                if x < framebuffer.width && y < framebuffer.height {
                    let shaded_color = (planet.shader)(&fragment, &uniforms);
                    framebuffer.set_current_color(shaded_color.to_hex());
                    framebuffer.point(x, y, fragment.depth);
                }
            }
        }
    
        
        let model_matrix = create_model_matrix(spaceship.position, spaceship.scale, spaceship.rotation.y);
    
        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: time_elapsed,
            noise: create_noise(),
        };
    
        let transformed_vertices = spaceship.vertices.iter()
            .map(|vertex| vertex_shader(vertex, &uniforms))
            .collect::<Vec<_>>();
    
        let triangles = transformed_vertices.chunks(3)
            .filter(|tri| tri.len() == 3)
            .map(|tri| [tri[0].clone(), tri[1].clone(), tri[2].clone()])
            .collect::<Vec<_>>();
    
        let mut fragments = Vec::new();
        for tri in &triangles {
            fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
        }
    
        for fragment in fragments {
            let (x, y) = (fragment.position.x as usize, fragment.position.y as usize);
            if x < framebuffer.width && y < framebuffer.height {
                let shaded_color = (spaceship.shader)(&fragment, &uniforms);
                framebuffer.set_current_color(shaded_color.to_hex());
                framebuffer.point(x, y, fragment.depth);
            }
        }
    
        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
        std::thread::sleep(frame_delay);
    }
    
}


fn handle_input(window: &Window, spaceship: &mut Spaceship, camera: &mut Camera) {
    let movement_speed = 0.5;
    let rotation_speed = 0.6;

    // Rotación de la nave
    if window.is_key_down(Key::A) {
        spaceship.rotate_left(rotation_speed);
    }
    if window.is_key_down(Key::D) {
        spaceship.rotate_right(rotation_speed);
    }

    // Rotación de la cámara
    if window.is_key_down(Key::A) {
        rotate_camera_around(&mut camera.eye, spaceship.position, rotation_speed);
    }
    if window.is_key_down(Key::D) {
        rotate_camera_around(&mut camera.eye, spaceship.position, -rotation_speed);
    }

    // Movimiento de la nave
    if window.is_key_down(Key::W) {
        spaceship.move_forward(movement_speed);
    }
    if window.is_key_down(Key::S) {
        spaceship.move_backward(movement_speed);
    }

    // Actualiza la cámara para que siempre apunte hacia la nave
    update_camera(camera, spaceship);
}


// Rotación de la cámara alrededor de la nave
fn rotate_camera_around(camera_position: &mut Vec3, center: Vec3, angle: f32) {
    let relative_position = *camera_position - center;
    let rotation_matrix = rotate_y(&Mat4::identity(), angle);
    let rotated_position = rotation_matrix.transform_vector(&relative_position);
    *camera_position = rotated_position + center;
}


fn update_camera(camera: &mut Camera, spaceship: &Spaceship) {
    camera.center = spaceship.position; // La cámara siempre mira hacia la nave
}
