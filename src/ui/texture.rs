// 骨架

use iced::widget::Text;
use iced::widget::image::Handle;
use iced::window;
use std::sync::OnceLock;

// 位图超采样倍率
const SS: f32 = 2.0;

// 机身尺寸
pub const BODY_W: f32 = 760.0;
pub const BODY_H: f32 = 500.0;
pub const FACE_X: f32 = 18.0;
pub const FACE_Y: f32 = 18.0;
pub const FACE_W: f32 = BODY_W - 2.0 * FACE_X;
pub const FACE_H: f32 = 460.0;
pub const FACE_R: f32 = 12.0;

// 应用图标
pub const ICON_PNG: &[u8] = include_bytes!("../../assets/icon.png");

pub struct Textures {
    pub body: Handle,
}

pub fn get() -> &'static Textures {
    static CELL: OnceLock<Textures> = OnceLock::new();
    CELL.get_or_init(|| Textures { body: body() })
}

pub fn window_icon() -> Option<window::Icon> {
    let img = image::load_from_memory(ICON_PNG).ok()?.to_rgb8();
    let (w, h) = img.dimensions();
    window::icon::from_rgba(img.into_raw(), w, h).ok()
}

pub fn body() -> Handle {
    // Handle::from_rgba(1, 1, vec![40, 24, 14, 255])
    bitmap(BODY_W, BODY_H, |x, y| {
        let mut c = wood(x, y);

        let t = y / BODY_H;
        let lacquer = 1.0 + 0.45 * (1.0 - smoothstep(0.0, 0.05, t))
            + 0.1 * (1.0 - smoothstep(0.0, 0.5, t))
            - 0.45 * smoothstep(0.7, 1.0, t);
        
        let (sd, _, _) = rounded_rect_sdf(x - FACE_X, y - FACE_Y - 4.0, FACE_W, FACE_H, FACE_R + 2.0);
        let edge = (x.min(BODY_W - x) / 50.0).min(1.0);
        let vignette = 0.6 + 0.4 * smooth(edge);
        let shadow = 1.0 - 0.75 * (1.0 - smoothstep(-6.0, 9.0, sd));
        let k = lacquer * vignette * shadow;
        for ch in &mut c {
            *ch *= k;
        }

        [c[0], c[1], c[2], 1.0]
    })
}


// ----- 噪声工具 ------

fn hash(x: i32, y: i32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343) ^ (y as u32).wrapping_mul(0xd816_3841);
    
    h = ( h ^ (h>>13)).wrapping_mul(0x85eb_ca6b);
    h ^= h >> 16;
    (h & 0x00ff_ffff) as f32 / 16_777_215.0
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn value_noise(x: f32, y: f32) -> f32 {
    let (xi, yi) = (x.floor() as i32, y.floor() as i32);
    let (xf, yf) = (smooth(x - x.floor()), smooth(y - y.floor()));

    let a = hash(xi, yi);
    let b = hash(xi + 1, yi);
    let c = hash(xi, yi + 1);
    let d = hash(xi + 1, yi + 1);

    let top = a + (b - a) * xf;
    let bottom = c + (d - c) * yf;

    top + (bottom - top) * yf
}

fn fbm(x: f32, y: f32, octaves: u32) -> f32 {
    let (mut sum, mut amp, mut freq, mut norm) = (0.0, 0.5, 1.0, 0.0);

    for _ in 0..octaves {
        sum += amp * value_noise(x * freq,  y * freq);
        norm += amp;
        amp *= 0.5;
        freq *= 2.03; 
    }

    sum / norm
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    smooth((x - e0) / (e1 - e0)).clamp(0.0, 1.0)
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn rgb(hex: u32) -> [f32; 3] {
    [
        ((hex >> 16) & 0xff) as f32 / 255.0,
        ((hex >> 8) & 0xff) as f32 / 255.0,
        (hex & 0xff) as f32 / 255.0,
    ]
}

fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

fn bitmap(w: f32, h: f32, shade: impl Fn(f32, f32) -> [f32; 4] + Sync) -> Handle {
    let (pw, ph) = ((w * SS) as usize, (h * SS) as usize);
    let mut px = vec![0u8; pw * ph * 4];
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let rows_per = ph.div_ceil(threads).max(1);

    std::thread::scope(|s| {
        for (chunk_idx, chunk) in px.chunks_mut(rows_per * pw * 4).enumerate() {
            let shade = &shade;
            s.spawn(move || {
                for (k, out) in chunk.chunks_exact_mut(4).enumerate() {
                    let (i, j) = (k % pw, chunk_idx * rows_per + k / pw);
                    let c = shade((i as f32 + 0.5) / SS, (j as f32 + 0.5) / SS);
                    out.copy_from_slice(&[to_u8(c[0]), to_u8(c[1]), to_u8(c[2]), to_u8(c[3])]);
                }
            });
        }
    });

    Handle::from_rgba(pw as u32, ph as u32, px)
}

fn rounded_rect_sdf(x: f32, y: f32, w: f32, h: f32, r: f32) -> (f32, f32, f32) {
    let (hx, hy) = (w / 2.0 - r, h / 2.0 - r);
    let (px, py) = (x - w / 2.0, y - h / 2.0);
    let (qx, qy) = (px.abs() - hx, py.abs() - hy);
    let outside = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    let d = outside + qx.max(qy).min(0.0) - r;

    let (nx, ny) = if qx > 0.0 && qy > 0.0 {
        (qx / outside * px.signum(), qy / outside * py.signum())
    } else if qx > qy {
        (px.signum(), 0.0)
    } else {
        (0.0, px.signum())
    };

    (d, nx, ny)
}

fn wood(x: f32, y: f32) -> [f32; 3] {
    let dark = rgb(0x1e_120a);
    let mid = rgb(0x4a2c19);
    let light = rgb(0x6e4526);

    let warp = fbm(x * 0.004, y * 0.012, 4) * 38.0;
    let ring = ((y + warp) * 0.11 + fbm(x * 0.02, y * 0.05, 2) * 2.0).sin();
    let figure = (0.5 + 0.5 * ring).powf(3.0);
    let band = fbm(x * 0.0025, y * 0.02 + 7.0, 3);
    let pore = smoothstep(0.78, 0.95, value_noise(x * 0.06, y * 1.1)) * 0.35;

    let mut c = mix(dark, mid, smoothstep(0.25, 0.75, band));
    c = mix(c, light, figure * 0.45 * band);
    let k = (0.92 + 0.08 * fbm(x * 0.3, y * 0.25, 2)) * (1.0 - pore);

    [c[0] * k, c[1] * k, c[2] * k]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn icon_is_png() {
        assert_eq!(&ICON_PNG[..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn icon_decodes() {
        let img = image::load_from_memory(ICON_PNG).unwrap().to_rgb8();
        let (w, h) = img.dimensions();
        assert_eq!(img.into_raw().len(), (w * h * 4) as usize);  // RGBA 每个像素4字节
    }
}
