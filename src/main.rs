use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() -> Result<(), Error> {
    env_logger::init();

    // Window initialization
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Rayquiem")
        .build(&event_loop)
        .unwrap();

    // Pixels buffer initialization
    let mut window_size = window.inner_size();
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    // Event loop
    let res = event_loop.run(|event, elwt| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => elwt.exit(),

            // Redraw frame
            WindowEvent::RedrawRequested => {
                draw(pixels.frame_mut(), window_size);
                if let Err(err) = pixels.render() {
                    println!("Failed to render frame: {err}");
                    elwt.exit();
                    return;
                }
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

            _ => {}
        },
        _ => {}
    });

    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

/// Draw the frame
fn draw(frame: &mut [u8], window_size: PhysicalSize<u32>) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % window_size.width as usize) as f64;
        let y = (i / window_size.width as usize) as f64;

        let px = x / window_size.width as f64;
        let py = y / window_size.height as f64;

        pixel.copy_from_slice(&[
            (255.0 * ((1.0 - px) * (1.0 - py))).round() as u8,
            (255.0 * (px * (1.0 - py))).round() as u8,
            (255.0 * ((1.0 - px) * py)).round() as u8,
            0xFF,
        ]);
    }
}
