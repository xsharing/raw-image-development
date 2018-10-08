
use ::rawloader;
use ::image;
use ::histogram::Histogram;

fn correct_gamma(c:f32) -> f32 {
    let c2;
    if c<=0.0031308*255.0 {
        c2 = 12.92*c;
    } else {
        c2 = (1.055*(c/255.0).powf(1.0/2.4)-0.055)*255.0;
    }
    return c2.max(0.0).min(255.0) as f32
}


pub fn main_logic(input: &str) {
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

    let mut hist_r_raw = Histogram::new();
    let mut hist_g_raw = Histogram::new();
    let mut hist_b_raw = Histogram::new();
    let mut hist_r = Histogram::new();
    let mut hist_g = Histogram::new();
    let mut hist_b = Histogram::new();
    let mut hist_x = Histogram::new();
    let mut hist_y = Histogram::new();
    let mut hist_z = Histogram::new();
    let mut hist_sr = Histogram::new();
    let mut hist_sg = Histogram::new();
    let mut hist_sb = Histogram::new();


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


            *pixel = image::Rgb([sr as u8, sb as u8, sg as u8]);

            hist_r_raw.increment(*r_raw as u64);
            hist_g_raw.increment(*g_raw as u64);
            hist_b_raw.increment(*b_raw as u64);
            hist_r.increment(r as u64);
            hist_g.increment(g as u64);
            hist_b.increment(b as u64);
            hist_x.increment(x as u64);
            hist_y.increment(y as u64);
            hist_z.increment(z as u64);
            hist_sr.increment(sr as u64);
            hist_sg.increment(sg as u64);
            hist_sb.increment(sb as u64);
        }
    }

    image.save("image.png").unwrap();

    debug!("{:?}, {:?}, {:?}", hist_r_raw.minimum(), hist_g_raw.minimum(), hist_b_raw.minimum());
    debug!("{:?}, {:?}, {:?}", hist_r.minimum(), hist_g.minimum(), hist_b.minimum());
    debug!("{:?}, {:?}, {:?}", hist_x.minimum(), hist_y.minimum(), hist_z.minimum());
    debug!("{:?}, {:?}, {:?}", hist_sr.minimum(), hist_sg.minimum(), hist_sb.minimum());


    debug!("{:?}, {:?}, {:?}", hist_r_raw.maximum(), hist_g_raw.maximum(), hist_b_raw.maximum());
    debug!("{:?}, {:?}, {:?}", hist_r.maximum(), hist_g.maximum(), hist_b.maximum());
    debug!("{:?}, {:?}, {:?}", hist_x.maximum(), hist_y.maximum(), hist_z.maximum());
    debug!("{:?}, {:?}, {:?}", hist_sr.maximum(), hist_sg.maximum(), hist_sb.maximum());
}