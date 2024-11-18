use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use std::time::{Duration, Instant};
use std::f32::consts::PI;

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
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: f32,
    noise: FastNoiseLite,
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

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    // Matrices de rotación y transformación combinadas.
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();
    

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );
    
    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );
    
    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;
    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );
    transform_matrix * rotation_matrix
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
    let mut window = Window::new("Laboratorio 4 - GPC", window_width, window_height, WindowOptions::default())
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


    let mut spaceship = Spaceship::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let mut camera = Camera::new(
        Vec3::new(0.0, 10.0, -20.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );


    

    
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut camera, &mut spaceship);
        framebuffer.clear();

        let time_elapsed = start_time.elapsed().as_secs_f32();
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        for planet in &planets {
            let model_matrix = create_model_matrix(planet.position, planet.scale, planet.rotation);
        
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time: time_elapsed,
                noise: create_noise(),
            };
        
            let transformed_vertices = planet.vertices.iter()
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
                    let shaded_color = (planet.shader)(&fragment, &uniforms);
                    framebuffer.set_current_color(shaded_color.to_hex());
                    framebuffer.point(x, y, fragment.depth);
                }
            }
        }
        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, camera: &mut Camera, spaceship: &mut Spaceship) {
    let movement_speed = 1.0;

    if window.is_key_down(Key::W) {
        spaceship.move_forward();
    }
    if window.is_key_down(Key::S) {
        spaceship.move_backward();
    }
    if window.is_key_down(Key::A) {
        spaceship.rotate_left();
    }
    if window.is_key_down(Key::D) {
        spaceship.rotate_right();
    }


    camera.follow_spaceship(&spaceship);
}