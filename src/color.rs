type Hex = u32;
type RGB = [u8; 3];
type HSV = [f64; 3];


// Converters
// To RGB
pub fn hsv_to_rgb(hsv: HSV) -> RGB {
    // according to wikipedia's article
    let [h, s, v] = hsv;
    let h = h % 360.0;
    
    let c = v * s;
    let h_ = h / 60.;
    let x = c * (1. - ((h_ % 2.0) - 1.).abs());

    let (r, g, b) = {
        let (r1, g1, b1) = {
            if h_ < 1. {
                (c, x, 0.)
            } else if h_ < 2. {
                (x, c, 0.)
            } else if h_ < 3. {
                (0., c, x)
            } else if h_ < 4. {
                (0., x, c)
            } else if h_ < 5. {
                (x, 0., c)
            } else {
                (c, 0., x)
            }
        };
        let m = v - c;
        let (r, g, b) = (r1 + m, g1 + m, b1 + m);
        ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
    };
    [r, g, b]
}

pub fn hex_to_rgb(hex: Hex) -> RGB {
    [
        (hex / 65536).try_into().unwrap(),
        ((hex % 65536) / 256).try_into().unwrap(),
        (hex % 256).try_into().unwrap()
    ]
}


// To hex
pub fn rgb_to_hex(rgb: RGB) -> Hex {
    rgb[0] as Hex * 65536 + rgb[1] as Hex * 256 + rgb[2] as Hex
}


pub fn hsv_to_hex(hsv: HSV) -> Hex {
    rgb_to_hex(hsv_to_rgb(hsv))
}


// To hsv
pub fn rgb_to_hsv(rgb: RGB) -> HSV {
    // according to the following page
    // https://www.had2know.org/technology/hsv-rgb-conversion-formula-calculator.html
    let &M = rgb.iter().max().unwrap();
    let &m = rgb.iter().min().unwrap();
    let V = M as f64 / 255.0;
    let S = if M == 0 { 0.0 } else { 1.0 - m as f64 / M as f64 };
    let H = {
        let [r, g, b] = rgb.map(|x| x as f64);
        let t = (
            (r - g / 2.0 - b / 2.0) / (r*r + g*g + b*b - r*g - r*b - g*b).sqrt()
        ).acos().to_degrees();
        if g >= b {
            t
        } else {
            360.0 - t
        }
    };
    [H, S, V]
}

pub fn hex_to_hsv(hex: Hex) -> HSV {
    rgb_to_hsv(hex_to_rgb(hex))
}
