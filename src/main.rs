use std::borrow::BorrowMut;

use image::{GenericImage, GenericImageView};

fn linear_to_nonlinear_srgb(val: f32) -> f32 {
    if val <= 0.0 {
        return val;
    }

    if val <= 0.0031308 {
        val * 12.92 // linear falloff in dark values
    } else {
        (1.055 * val.powf(1.0 / 2.4)) - 0.055 // gamma curve in other area
    }
}

fn nonlinear_to_linear_srgb(val: f32) -> f32 {
    if val <= 0.0 {
        return val;
    }
    if val <= 0.04045 {
        val / 12.92 // linear falloff in dark values
    } else {
        ((val + 0.055) / 1.055).powf(2.4) // gamma curve in other area
    }
}

fn main() {
    // Please note: This code is not good! It's good enough for demonstration purposes.

    let mut img = image::open("spineboy-eye-pma.png").unwrap();
    let pixels = img.clone();

    for (x, y, mut pixel) in pixels.pixels() {
        let a: f32 = if pixel.0[3] > 0 {
            f32::from(pixel.0[3]) / 255.0
        } else {
            0.0
        };

        let mut r: f32 = if pixel.0[0] > 0 {
            f32::from(pixel.0[0]) / 255.0
        } else {
            0.0
        };
        r /= a;
        r = nonlinear_to_linear_srgb(r);
        r *= a;
        r = linear_to_nonlinear_srgb(r);

        let mut g: f32 = if pixel.0[1] > 0 {
            f32::from(pixel.0[1]) / 255.0
        } else {
            0.0
        };
        g /= a;
        g = nonlinear_to_linear_srgb(g);
        g *= a;
        g = linear_to_nonlinear_srgb(g);

        let mut b: f32 = if pixel.0[2] > 0 {
            f32::from(pixel.0[2]) / 255.0
        } else {
            0.0
        };
        b /= a;
        b = nonlinear_to_linear_srgb(b);
        b *= a;
        b = linear_to_nonlinear_srgb(b);

        pixel.0[0] = f32::ceil(r * 255.0) as u8;
        pixel.0[1] = f32::ceil(g * 255.0) as u8;
        pixel.0[2] = f32::ceil(b * 255.0) as u8;

        img.put_pixel(x, y, pixel);
    }

    img.save("spineboy-eye-corrected-pma.png").unwrap();
}
