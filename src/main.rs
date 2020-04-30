use minifb;
use minifb::WindowOptions;

mod Classes;
use Classes::Vec3::Vec3;
use Classes::Rgb::Rgb;

fn main() {
    let width = 1200;
    let height = 800;
    let mut window = Classes::Window::Window{
        width,
        height,
        primary_buffer: vec![0; width * height],
        secondary_buffer: vec![0; width * height]
    };
    let mut minifb_window = minifb::Window::new(
        "RayTracer",
        width,
        height,
        WindowOptions::default()
    )
        .unwrap_or_else(|e| {
            panic!("{}", e)
        });

    while minifb_window.is_open() && !minifb_window.is_key_down(minifb::Key::Escape) {

        let mut index = 0;
        for i in 0..window.height {
            for x in 0..window.width {
                let color1 = Rgb {
                    r: 255.0,
                    g: 255.0,
                    b: 255.0
                };
                let color2 = Rgb {
                    r: 120.0,
                    g: 120.0,
                    b: 255.0
                };
                let mixed = color1.mix(color2, i as f32 / window.height as f32);

                window.secondary_buffer[index] = mixed.to_int();
                index += 1;
            }
        }



        window.swap_buffers();
        minifb_window.update_with_buffer(&window.primary_buffer, width, height)
            .unwrap();
    }


}