use image::{ColorType, ImageError};
use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

/// limitで指定した繰り返し回数を上限に、半径2の円から出るか出ないかをチェックする
/// cがマンデルブロ集合に含まれてそうなら None を返す
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
/// 与えられた文字列を separator で分割して、指定の型に変換して返す
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(
        parse_pair::<i32>("10,20", ','),
        Some::<(i32, i32)>((10, 20))
    );
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(
        parse_pair::<f64>("0.5x1.5", 'x'),
        Some::<(f64, f64)>((0.5, 1.5))
    );
}

/// 浮動小数点のペアを複素数にして返す
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// bounds は全体サイズ
/// pixel は特定のピクセル
/// ul, lr は複素平面の描画範囲
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

/// マンデルブロ集合に含まれる場合は黒(0)
/// それ以外の場合は、円から抜け出すまでの回数が少ないほど暗い色になる
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.0 {
        for col in 0..bounds.1 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

/// PNGEncoder の .encode は deprecated になっていたので雑に書き換え
/// Pngに書き出せればええじゃろ・・・
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {
    image::save_buffer(
        filename,
        pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::L8,
    )
}
