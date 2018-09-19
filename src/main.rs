#[macro_use] extern crate log;
extern crate clap;
use clap::{Arg, App};
extern crate loggerv;
extern crate rawloader;
extern crate image;

fn correct_gamma(c:f32) -> f32 {
    let c2;
    if c<=0.0031308*255.0 {
        c2 = 12.92*c;
    } else {
        c2 = (1.055*(c/255.0).powf(1.0/2.4)-0.055)*255.0;
    }
    return c2.max(0.0).min(255.0)
}

fn main() {
    let args = App::new("raw image development")
        .version("0.0.1")
        .author("Nobuyuki Horiuchi <horiuchinobuyuki@gmail.com>")
        .about("wip..")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("input")
            .help("input file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    loggerv::init_with_verbosity(args.occurrences_of("v")).unwrap();

    let input = args.value_of("input").unwrap();
    debug!("input file: {}", input);

    let raw = rawloader::decode_file(input).unwrap();
    debug!("size and orientation: ({}, {}), {:?}", raw.width, raw.height, raw.orientation);
    debug!("maker and model: ({}, {})", raw.clean_make, raw.clean_model);
    debug!("cfa and cpp: {:?}, {:?}, {}", raw.cfa, raw.cropped_cfa(), raw.cpp);
    debug!("blacklevels, whitelevels, wb_coeffs: {:?} {:?} {:?}", raw.blacklevels, raw.whitelevels, raw.wb_coeffs);
    debug!("{:?}", raw.cam_to_xyz());

    debug!("crops: {:?}", raw.crops);

    let cam_to_xyz = raw.cam_to_xyz();

    let mut image = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new((raw.width as u32)/2, (raw.height as u32)/2);

    //RGGB
    let r_offset = 0;
    let g_offset = 1;
    let b_offset = raw.width as u32;

    let r_blacklevel = raw.blacklevels[0] as f32;
    let g_blacklevel = raw.blacklevels[1] as f32;
    let b_blacklevel = raw.blacklevels[2] as f32;
    let r_whitelevel = raw.whitelevels[0] as f32;
    let g_whitelevel = raw.whitelevels[1] as f32;
    let b_whitelevel = raw.whitelevels[2] as f32;
    let coeff_sum = raw.wb_coeffs[0] + raw.wb_coeffs[1] + raw.wb_coeffs[2];

    let mut r_max = 0.0;
    let mut g_max = 0.0;
    let mut b_max = 0.0;
    let mut r_min = 1000.0;
    let mut g_min = 1000.0;
    let mut b_min = 1000.0;


    let mut x_max = 0.0;
    let mut y_max = 0.0;
    let mut z_max = 0.0;
    let mut x_min = 1000.0;
    let mut y_min = 1000.0;
    let mut z_min = 1000.0;

    if let rawloader::RawImageData::Integer(data) = raw.data {
        debug!("length: {}", data.len());

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let r_raw = data.get((y*2*(raw.width as u32)+x*2+r_offset) as usize).unwrap();
            let g_raw = data.get((y*2*(raw.width as u32)+x*2+g_offset) as usize).unwrap();
            let b_raw = data.get((y*2*(raw.width as u32)+x*2+b_offset) as usize).unwrap();

            let r = ((*r_raw as f32 - r_blacklevel)/r_whitelevel) *255.0 * raw.wb_coeffs[0] / coeff_sum * 3.0;
            let g = ((*g_raw as f32 - g_blacklevel)/g_whitelevel) *255.0 * raw.wb_coeffs[1] / coeff_sum * 3.0;
            let b = ((*b_raw as f32 - b_blacklevel)/b_whitelevel) *255.0 * raw.wb_coeffs[2] / coeff_sum * 3.0;

            let x = cam_to_xyz[0][0]*r + cam_to_xyz[0][1]*g + cam_to_xyz[0][2]*b;
            let y = cam_to_xyz[1][0]*r + cam_to_xyz[1][1]*g + cam_to_xyz[1][2]*b;
            let z = cam_to_xyz[2][0]*r + cam_to_xyz[2][1]*g + cam_to_xyz[2][2]*b;


            let sr = correct_gamma(x*3.2406-y*1.5372-z*0.4986);
            let sg = correct_gamma(-x*0.9689+y*1.8758+z*0.0415);
            let sb = correct_gamma(x*0.0667-y*0.2040+z*1.0570);

            r_max = f32::max(r, r_max);
            b_max = f32::max(g, g_max);
            g_max = f32::max(b, b_max);

            r_min = f32::min(r, r_min);
            b_min = f32::min(g, g_min);
            g_min = f32::min(b, b_min);


            x_max = f32::max(x, x_max);
            y_max = f32::max(y, y_max);
            z_max = f32::max(z, z_max);

            x_min = f32::min(x, x_min);
            y_min = f32::min(y, y_min);
            z_min = f32::min(z, z_min);

//            debug!("{}, {}, {}", r, g, b);

            *pixel = image::Rgb([sr as u8, sb as u8, sg as u8])
        }
    }

    image.save("image.png").unwrap();

    debug!("{}, {}, {}", r_max, g_max, b_max);
    debug!("{}, {}, {}", r_min, g_min, b_min);
    debug!("{}, {}, {}", x_max, y_max, z_max);
    debug!("{}, {}, {}", x_min, y_min, z_min);
}

