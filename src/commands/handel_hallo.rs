use std::io::{self, Write};
use image::{GenericImageView, imageops::FilterType};

pub fn splash_screen() {
    print!("\x1B[2J\x1B[1;1H");

    let image_bytes = include_bytes!("../logo/logo.jpg");

    let img = image::load_from_memory(image_bytes)
        .expect("Failed to parse embedded logo.jpg bytes");

    let target_width = 56;
    let target_height = 112; 
    let resized_img = img.resize(target_width, target_height, FilterType::Lanczos3);

    let img_width = resized_img.width();
    let img_height = resized_img.height();
    
    let mut shape_grid = vec![vec![false; img_width as usize]; img_height as usize];

    for y in 0..img_height {
        for x in 0..img_width {
            let pixel = resized_img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            if r > 45 || g > 45 || b > 45 {
                shape_grid[y as usize][x as usize] = true;
            }
        }
    }

    let mut logo_rows: Vec<String> = Vec::new();

    for y in (0..img_height as usize).step_by(4) {
        let mut row_string = String::new();
        for x in (0..img_width as usize).step_by(2) {
            
            let mut code = 0;
            let mut active_dots = 0;
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;

            macro_rules! sample_dot {
                ($dy:expr, $dx:expr, $bit:expr) => {
                    let ny = y + $dy;
                    let nx = x + $dx;
                    if shape_grid.get(ny).and_then(|r| r.get(nx)).copied().unwrap_or(false) {
                        code |= $bit;
                        let pixel = resized_img.get_pixel(nx as u32, ny as u32);
                        r_sum += pixel[0] as u32;
                        g_sum += pixel[1] as u32;
                        b_sum += pixel[2] as u32;
                        active_dots += 1;
                    }
                };
            }

            sample_dot!(0, 0, 0x01);
            sample_dot!(1, 0, 0x02);
            sample_dot!(2, 0, 0x04);
            sample_dot!(0, 1, 0x08);
            sample_dot!(1, 1, 0x10);
            sample_dot!(2, 1, 0x20);
            sample_dot!(3, 0, 0x40);
            sample_dot!(3, 1, 0x80);

            if code == 0 {
                row_string.push_str(" ");
            } else {
                let r_avg = r_sum / active_dots;
                let g_avg = g_sum / active_dots;
                let b_avg = b_sum / active_dots;

                let max_channel = r_avg.max(g_avg).max(b_avg);
                
                let (r_bright, g_bright, b_bright) = if max_channel > 0 {
                    let factor = 255.0 / max_channel as f32;
                    (
                        ((r_avg as f32 * factor) as u32).min(255),
                        ((g_avg as f32 * factor) as u32).min(255),
                        ((b_avg as f32 * factor) as u32).min(255),
                    )
                } else {
                    (r_avg, g_avg, b_avg)
                };

                let braille_char = std::char::from_u32(0x2800 + code).unwrap_or(' ');
                row_string.push_str(&format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r_bright, g_bright, b_bright, braille_char));
            }
        }
        logo_rows.push(row_string);
    }

    

    let info_lines = vec![
        "\x1B[1;35mWelcome to the halloShell!\x1B[0m".to_string(),
        "-----------------------------".to_string(),
        "".to_string(),
        format!("\x1B[1;36mShell:\x1B[0m      HALLO SYSTEM v1.0.0"),
        format!("\x1B[1;36mEngine:\x1B[0m     Built in Rust"),
        format!("\x1B[1;36mStatus:\x1B[0m     Operational & Ready"),
        format!("\x1B[1;36mPrompt:\x1B[0m     hallo->"),
        "".to_string(),
        "  \x1B[41m   \x1B[42m   \x1B[43m   \x1B[44m   \x1B[45m   \x1B[46m   \x1B[0m".to_string(),
    ];

    println!();

    let total_rows = logo_rows.len().max(info_lines.len());
    let start_offset = if logo_rows.len() > info_lines.len() {
        (logo_rows.len() - info_lines.len()) / 2
    } else {
        0
    };

    for i in 0..total_rows {
        let logo_row = logo_rows.get(i).map(|s| s.as_str()).unwrap_or("");
        
        let info_row = if i >= start_offset && (i - start_offset) < info_lines.len() {
            &info_lines[i - start_offset]
        } else {
            ""
        };

        println!("   {}   {}", logo_row, info_row);
    }

    println!("\n");
    io::stdout().flush().unwrap();
}
