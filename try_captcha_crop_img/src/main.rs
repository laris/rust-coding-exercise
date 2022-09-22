
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::env;
use image::*;
use imageproc::*;

fn main() {
    // args
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    println!("input file: {:#?}", file_name);
    // open file name
    let img = image::open(file_name).unwrap();
    //println!("Open file {:?}\n {:?}", file_name, img);
    // convert to luma8, 
    // If the image was already the correct format, it is returned as is. Otherwise, a copy is created.
    let mut img_luma8 = img.into_luma8();
    let img_otsu_level = imageproc::contrast::otsu_level(&img_luma8);
    imageproc::contrast::threshold_mut(&mut img_luma8, img_otsu_level);
    //imageproc::contrast::stretch_contrast_mut(&mut img_luma8, 127, 128);
    //println!("img_luma8:\n{:?}", img_luma8);
    //println!("histogram: {:?}", imageproc::stats::histogram(&img)[0]);
    //let percentile = imageproc::stats::percentile(&img, 10);
    //println!("percentile: {:?}", percentile);
    //let mut img = imageproc::contrast::adaptive_threshold(&img, 3);
    //let mut img_luma8 = imageproc::contrast::adaptive_threshold(&img_luma8, 3);
    //println!("Open file {:?}\ninto_luma8:\n{:?}", file_name, img);
    //let img_dimg = DynamicImage::ImageLuma8(img_luma8);
    let file_name_to_save = format!("{}.pbm", file_name);
    img_luma8.save_with_format(file_name_to_save, 
                               image::ImageFormat::Pnm).unwrap();
    // save ImageBuffer to file
//    image::save_buffer_with_format(
//            "save_buffer_with_format.jpg", 
//            &img_luma8, //.as_raw(),
//            186, 72, ColorType::L8, ImageFormat::Jpeg
//        );

// Ok(ImageRgb8(ImageBuffer { width: 186, height: 72, _phantom: PhantomData, data:

    //println!("guess: \n{:?}", image::guess_format(&img_luma8));
    /*
    match img_luma8 {
        //DynamicImage::ImageLuma8(img_buf) => { 
        DynamicImage::ImageLuma8(img_buf) => { 
            //println!("data container: \n{:?}", img_buf.as_raw());
            println!("guess: \n{:?}", image::guess_format(&img_buf.as_raw()));
            //let file_name_to_save = format!("{}-save-with.bmp", file_name);
            //image::save_buffer_with_format(
            //        file_name_to_save, 
            //        &img_buf.as_raw(),
            //        186, 72, ColorType::L8, ImageFormat::Bmp
            //    );
        },
        _ => () ,
    }
 */


    // check image
    //println!("Open file {:#?}\n {:?}", file_name, img.unwrap());
    // ImageLuma8(ImageBuffer { width: 186, height: 72, _phantom: PhantomData, data:
    //println!("guess_format: {:?}", image::guess_format(&img.0.as_raw()));

    //let img_luma8 = img.into_luma8();
    //let img_luma8 = img.clone();
    //let img_luma8 = img;
    //println!("After into_luma8 and return ImageBuffer\n {:?}", img_luma8);
    // ImageBuffer { width: 186, height: 72, _phantom: PhantomData, data:
    //let file_name_img_save = format!("{}.bmp", file_name);
    //img_luma8.save_with_format(file_name_img_save, 
    //                            image::ImageFormat::Bmp).unwrap();

    //println!("dimensions: {:?}", img_luma8.dimensions());

    // try to crop left half of img and save to file
/*
    let mut img_luma8_obj = DynamicImage::new_luma8(
                            img_luma8.width(), 
                            img_luma8.height());
    let file_name_save = format!("{}-obj.jpg", file_name);
    img_luma8_obj.save_with_format(
                file_name_save, 
                image::ImageFormat::Jpeg).unwrap();
 */

/*
    //let mut img_crop1 = DynamicImage::ImageLuma8(img_luma8.clone())
    let mut img_crop1 = img_dimg
                            .crop_imm(0, 0, 
                            img_dimg.width()/2, 
                            img_dimg.height());
    let file_name_save = format!("{}-crop1.jpg", file_name);
    img_crop1.save_with_format(
                file_name_save, 
                image::ImageFormat::Jpeg).unwrap();

    //let mut img_crop2 = DynamicImage::ImageLuma8(img_dimg.clone())
    let mut img_crop2 = img_dimg
                            .crop_imm(
                                img_dimg.width()/2, 
                                0, 
                                img_dimg.width()/2, 
                                img_dimg.height()
                            );
    let file_name_save = format!("{}-crop2.jpg", file_name);
    img_crop2.save_with_format(
                file_name_save, 
                image::ImageFormat::Jpeg).unwrap();
 */    
    // define cut pair tuple with [x, x]
    // define container to save cut_xx
    let mut cut_list: Vec<cut_xx> = Vec::new();
    // get x points vs black accumulated numbers mapping
    let x_points_histo = find_x_points(&img_luma8);
    println!("x_points: {:?}", x_points_histo);
    // find left and right not-black edge x-axis, find left then right
    let x_edge = find_x_left_right(&x_points_histo);
    println!("cut_xx({}, {})", x_edge.0, x_edge.1);
    // crop the img between edge
    //crop_save(&img_luma8, &x_edge, file_name,"between_edge".to_string());
    println!("average char width: {}", get_average_char_width(&x_edge));
    println!("cut 4 average: {:?}", cut_4_char_average(&x_edge));
/*
    // cut 1st
    let mut cp1;
    let mut cp2 = cut_xx(0,0);
    if let Some(cp) = find_cut_points_1time(&x_points_histo, &x_edge) {
        println!("cut, {:?}", cp);
        cp1 = cp.0;
        cp2 = cp.1;
    }
    // cut 2nd
    match find_cut_points_1time(&x_points_histo, &cp2) {
        Some(cp) => println!("cut, {:?}", cp),
        None => println!("None"),
    }
    //println!("len:{:?}", find_x_cut_points_between_edge(x_edge));
*/

} // end of main

#[derive(Debug, Clone)]
struct cut_xx(u32, u32);

// return vec map the x[0..width] -> black point num
fn find_x_points(img: &GrayImage) -> Vec::<usize> {
    let mut black_x = vec![0; img.width() as usize];
    for pix in img.enumerate_pixels() {
        match pix {
            (x, y, luma) => {
                //println!("{:?}", luma[0]);
                for x_idx_inner in 0..img.width() as usize {
                    if x_idx_inner == x as usize && luma[0] == 0 {
                        black_x[x_idx_inner] += 1;
                    }
                }
            }
            _ => {}
        }
    }
    black_x
    //for x_idx in 0..img.width() as usize {
        //println!("x: {}, black: {}", x_idx, black_x[x_idx]);
    //}

}

// return a tuple (left, right) between edge for valid black point
fn find_x_left_right(x_points_list: &Vec::<usize>) -> cut_xx {
    let mut x1 = 0;
    let mut x2 = 0;
    for x_idx in 0..x_points_list.len() as usize {
        if x_points_list[x_idx] != 0 {
            x1 = x_idx;
            break
        }
    }
    for x_idx in (0..x_points_list.len() as usize).rev() {
        if x_points_list[x_idx] != 0 {
            x2 = x_idx;
            break
        }
    }
    cut_xx(x1 as u32, x2 as u32)
}
/*
fn find_x_cut_points_between_edge(cut_range: cut_xx) -> Vec::<cut_xx> {
    let len = cut_range.1 - cut_range.0 + 1;
    let cut_list: Vec::<cut_xx> = Vec::new();
    if len > 0 {
        println!("len: {:?}", len);

    } else {
        println!("Err: len <= 0");
    }
}
*/

// in: x-black_points, range, out -> status, first cut part pair, left range
fn find_cut_points_1time(x_list: &Vec::<usize>, cut_range: &cut_xx) -> 
    Option<(cut_xx, cut_xx)>
    // None, no cut point, 
    // Some() find 1 point + first cut list + left 
{
    let mut l = 0;
    let mut r = 0;
    let mut f = false;
    for i in cut_range.0 as usize ..=cut_range.1 as usize {
        if x_list[i] > 0 && !f {
            l = i;
            f = true;
        }
        if x_list[i] == 0 && f {
            r = i;
            f = false;
            break; // stop search
        }
    }
    if l != 0 && r != 0 && l != r && r != cut_range.1 as usize {
        return Some((cut_xx(l as u32,r as u32), cut_xx(r as u32+1,cut_range.1)))
    } else if l == cut_range.0 as usize && r == cut_range.1 as usize {
        return None
    } else { None }
}

fn crop_save(img: &GrayImage, x: &cut_xx, base_name: &String, fname: String) {
    let mut img_crop = DynamicImage::ImageLuma8(img.clone());
    println!("cut range: x: {}, y: {}, w: {}, h: {}", x.0, 0, x.1 - x.0 +1, img_crop.height());
    let img_cropped = img_crop.crop_imm(
        x.0,
        0,
        x.1 - x.0 + 1,
        img_crop.height()
    );
    let file_name_save = format!("{}-crop-{}.pbm", base_name, fname);
    img_cropped.save_with_format(
                file_name_save, 
                image::ImageFormat::Pnm).unwrap();
}

fn get_average_char_width(x_edge: &cut_xx) -> u32 {
    let width = x_edge.1 - x_edge.0 + 1;
    width / 4
}

fn cut_4_char_average(x_edge: &cut_xx) -> Vec::<cut_xx> {
    let w = get_average_char_width(x_edge);
    vec![cut_xx(0,0),cut_xx(0,0)]
}