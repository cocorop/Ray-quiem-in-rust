use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

#[path = "lib/utils/common.rs"]
mod common;
#[path = "lib/hittable/hittable.rs"]
mod hittable;
#[path = "lib/hittable/hittable_list.rs"]
mod hittable_list;
#[path = "lib/utils/ray.rs"]
mod ray;
#[path = "lib/hittable/sphere.rs"]
mod sphere;
#[path = "lib/utils/vec3.rs"]
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const FOCAL_LENGTH: f64 = 1.0;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (WIDTH as f64 / HEIGHT as f64);

fn main() -> Result<(), Error> {
    env_logger::init();

    // Window initialization
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Rayquiem")
        .with_resizable(false)
        .with_inner_size(PhysicalSize {
            width: WIDTH,
            height: HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    // Pixels buffer initialization
    let mut window_size = window.inner_size();
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    // Camera
    let mut camera_center = Vec3::ZERO;

    const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = VIEWPORT_U / (WIDTH as f64);
    let pixel_delta_v = VIEWPORT_V / (HEIGHT as f64);

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;
    let pixel0_pos = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Event loop
    let res = event_loop.run(|event, elwt| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => elwt.exit(),

            // Redraw the frame
            WindowEvent::RedrawRequested => {
                let start = Instant::now();

                draw(
                    pixels.frame_mut(),
                    window_size,
                    pixel0_pos,
                    pixel_delta_u,
                    pixel_delta_v,
                    camera_center,
                    &world,
                );

                let duration = start.elapsed();

                if let Err(err) = pixels.render() {
                    println!("Failed to render frame: {err}");
                    elwt.exit();
                    return;
                }

                println!(
                    "Rendered frame in {:?} - Camera: {}",
                    duration, camera_center
                );
            }

            // Resize window
            WindowEvent::Resized(new_size) => {
                if let Err(err) = pixels.resize_surface(new_size.width, new_size.height) {
                    println!("Failed to resize surface: {err}");
                    elwt.exit();
                    return;
                }

                if let Err(err) = pixels.resize_buffer(new_size.width, new_size.height) {
                    println!("Failed to resize pixels buffer: {err}");
                    elwt.exit();
                    return;
                }

                window_size = new_size;
                window.request_redraw();
            }

            // Handle keyboard inputs
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
                // Redraw frame
                KeyCode::KeyR => window.request_redraw(),

                // Camera movement
                KeyCode::KeyW => {
                    camera_center.y += 0.1;
                    window.request_redraw();
                }
                KeyCode::KeyA => {
                    camera_center.x -= 0.1;
                    window.request_redraw();
                }
                KeyCode::KeyS => {
                    camera_center.y -= 0.1;
                    window.request_redraw();
                }
                KeyCode::KeyD => {
                    camera_center.x += 0.1;
                    window.request_redraw();
                }
                _ => {}
            },

            _ => {}
        },
        _ => {}
    });

    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

/// Auxiliary function to draw the frame
fn draw(
    frame: &mut [u8],
    window_size: PhysicalSize<u32>,
    pixel0_pos: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    camera_center: Vec3,
    world: &dyn Hittable,
) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % window_size.width as usize) as f64;
        let y = (i / window_size.width as usize) as f64;

        let pixel_center = pixel0_pos + x * pixel_delta_u + y * pixel_delta_v;
        let ray = Ray {
            origin: camera_center,
            direction: pixel_center - camera_center,
        };

        let color = ray_color(ray, world);
        pixel.copy_from_slice(&[
            color.x.floor() as u8,
            color.y.floor() as u8,
            color.z.floor() as u8,
            0xFF,
        ]);
    }
}

fn ray_color(ray: Ray, world: &dyn Hittable) -> Vec3 {
    let normalized = ray.direction.normalize();
    let linearized = normalized.y / 2.0 + 0.5;

    let mut hit_record = HitRecord::new();
    if world.hit(ray, 0.0, 50.0, &mut hit_record) {
        return (hit_record.normal + Vec3::ONE) * 127.5; // divide by 2 then multiply by 255
    }

    // Blue-white gradient
    let col = (1.0 - linearized) * Vec3::ONE + linearized * Vec3::new(0.5, 0.7, 1.0);
    col * 255.0
}
