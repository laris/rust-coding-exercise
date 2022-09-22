#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::env;
//use substring::Substring;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, RgbaImage, Rgba, GrayImage};
use imageproc::contrast::threshold;
//use palette::{FromColor, Hsl, IntoColor, Lch, Srgb};
extern crate histo_fp;
use histo_fp::Histogram;

fn main() {
    //println!("Hello, world!");
    // input file name and parameters
    struct arg_input {
        path: std::path::PathBuf,
        param: String,
    }
    let file_name = env::args().nth(1).expect("no file name");
    let param     = env::args().nth(2).expect("no param");
    let args = arg_input {
        path: std::path::PathBuf::from(file_name),
        param: param,
    };
    let filter_param = args.param.parse::<f32>().unwrap();
    //println!("{:#?} {:#?}", args.path, args.param.parse::<f32>().unwrap());
    // base file name
    let fname_base: String = args.path.file_name().unwrap().to_str().unwrap().into();
    let fn_base = fname_base.get(..11);
    //println!("{:#?}", fn_base.unwrap());

    // start images business
    let img = image::open(args.path).unwrap();
    //println!("dimensions: {:?}", img.dimensions());
    //println!("color: {:?}", img.color());

    // copyimg_new from img
    let mut img_new = img.clone();
    // iterate pixel
    for pixel in img.pixels() {
        // get bool value between blue and 2 x green
        let blue_ge_2x_green = 
            rgb_filter_blue_ge_2xgreen(pixel.2[2], pixel.2[1], pixel.2[0], filter_param);
        // debug pixel
        /*
        println!("raw: {:?}, x: {:3}, y: {:3}, rgb: {:3}, {:3}, {:3}, Bge2xG: {}", 
            pixel,
            pixel.0, pixel.1, 
            pixel.2[0], pixel.2[1], pixel.2[2], 
            blue_ge_2x_green);
         */

        if !blue_ge_2x_green {
            //new.put_pixel(pixel.0, pixel.1, image::Rgba([255,255,255,255]))
            //no color
           img_new.put_pixel(pixel.0, pixel.1, image::Rgba([255,255,255,0]))
        }
    }

    // save image toimg_new
    let fn_new = format!("{base}{post}{param}{ext}", 
            base = fn_base.unwrap(), 
            post = "post-", 
            param = args.param, 
            ext = ".jpg");
    //println!("{:?}",fn_new);
    img_new.save(&fn_new).unwrap();

    // grayscale
    let img_grayscale = img_new.grayscale();
    let fn_grayscale = format!("{base}{post}gray-{param}{ext}", 
            base = fn_base.unwrap(), 
            post = "post-", 
            param = args.param, 
            ext = ".jpg");
    //println!("{:?}",fn_grayscale);
    img_grayscale.save(&fn_grayscale).unwrap();

    // tesseract
    let img_target = image::open(&fn_new).unwrap();
    
/* 
    //let result = tesseract::ocr(&fn_new.as_str(), "eng");
    //let result = tesseract::ocr("image-4g62-post-1.2.jpg", "eng");
    let result = tesseract::ocr("image-8b2g-post-1.2.jpg", "eng");
    println!("{:?}", result);
*/
    let mut img_luma = img_target.into_luma();
    // luma grayscale
    //let mut img_luma = imageproc::contrast::adaptive_threshold(&img_luma, 4);
    //imageproc::contrast::equalize_histogram_mut(&mut img_luma);
    let img_otsu_level = imageproc::contrast::otsu_level(&img_luma);
    //imageproc::contrast::threshold_mut(&mut img_luma, img_otsu_level);
    println!("otsu_level: {}", img_otsu_level);
    println!("percentile: {}", imageproc::stats::percentile(&img_luma, 10));
    //imageproc::contrast::stretch_contrast_mut(&mut img_luma, 100, 200);

    //let mut img_luma = imageproc::contours::find_contours(&img_luma);
    let mut img_luma = imageproc::contrast::adaptive_threshold(&img_luma, 3);
    use imageproc::distance_transform::Norm;
    imageproc::morphology::dilate_mut(&mut img_luma, Norm::L1, 1);
    imageproc::morphology::erode_mut(&mut img_luma, Norm::L1, 1);
    
    //imageproc::contrast::threshold_mut(&mut img_luma, 100);
    // err println!("histogram: {:#?}", imageproc::stats::histogram(&img_luma));

    // dither and binarized the grayscale
    //image::imageops::dither(&mut img_luma, &image::imageops::BiLevel);
    let fn_luma = format!("{base}{post}luma-{param}{ext}", 
            base = fn_base.unwrap(), 
            post = "post-", 
            param = args.param, 
            ext = ".bmp");
    //println!("{:?}",fn_luma);
    img_luma.save(&fn_luma).unwrap();

    // cut 
    // find edge in x
    let w = img_luma.width();
    let h = img_luma.height();
    println!("width: {}, height: {}", w, h);

    let mut black_x = vec![0; w as usize];
    for pix in img_luma.enumerate_pixels() {
        //println!("pix: {:?}", pix);
        //match (x: u8, y: u8, luma: image::Luma) {
        match pix {
            (x, y, luma) => { 
                //println!("{:}", luma[0]);
                for i in 0..w as usize {
                    if i == x as usize && luma[0] == 0 {
                        black_x[i] += 1;
                    }
                } 
            },
            _ => {}
        }
    }

    #[derive(Debug, Clone)]
    struct cut_pair(u32, u32);
    let mut cp: cut_pair = cut_pair(0,0);
    let mut x_left = 0;
    let mut x_right = 0;
    let mut flag = false;
    let mut cut_list: Vec<cut_pair> = Vec::new();
    for i in 0..w {
        if black_x[i as usize] > 0 && !flag
        {
            x_left = i;
            cp.0 = i;
            println!("lx: {:?}", x_left);
            flag = true;
        }
        if black_x[i as usize] == 0 && flag {
            x_right = i - 1;
            cp.1 = i - 1;
            println!("rx: {:?}", x_right);
            flag = false;
            cut_list.push(cp.clone());
        }
        println!("{:>3}| {:*<2$}", i, "", size=black_x[i as usize]);
    }
    println!("cut pair count: {:?}, {:?}", cut_list.len(), cut_list);
    // get average char width
    let cut_list_size = cut_list.len();
    let mut width_both_chars = 0;
    for i in &cut_list {
        println!("{:?} - {:?}", i.0, i.1);
        width_both_chars += i.1 - i.0 + 1;
    }
    let mut width_average_per_char = width_both_chars/4;
    println!("average both chars: {}, char width: {}", 
                width_both_chars, width_average_per_char);

    // cut the images
    // 定义一个百分比来过滤字符宽度
    let param_fuse = 0.3;
    //let mut img_sub1: image::ImageBuffer;
    //let mut img_sub2: image::SubImage<image::DynamicImage::ImageLuma8(GrayImage)>;
    //let mut img_sub3: image::SubImage<image::DynamicImage::ImageLuma8(GrayImage)>;
    //let mut img_sub4: image::SubImage<image::DynamicImage::ImageLuma8(GrayImage)>;
        if        cut_list_size == 1 {
            println!("cut to {} x images", cut_list_size);
            let mut img_luma_clone = img_luma;
            let mut img_sub1 = image::imageops::crop(
                        &mut img_luma_clone,
                        cut_list[0].0,
                        0,
                        cut_list[0].1 - cut_list[0].0,
                        185
                    );
            let img_sub1_clone = img_sub1;
            img_sub1_clone.to_image().save("img_sub1.bmp").unwrap();
            if  ((cut_list[0].1 - cut_list[0].0) as f32) > (30.0 * (1.0-param_fuse)) && 
                ((cut_list[0].1 - cut_list[0].0) as f32) < (30.0 * (1.0+param_fuse))
                {
                    println!("single char");
                }
            else if ((cut_list[0].1 - cut_list[0].0) as f32) > (2.0 * 30.0 * (1.0-param_fuse)) && 
                    ((cut_list[0].1 - cut_list[0].0) as f32) < (2.0 * 30.0 * (1.0+param_fuse))
                {
                    println!("two char");
                    let mut img_sub1_clone = img_sub1;
                    let img_sub1_sub1 = image::imageops::crop(
                        &mut img_sub1_clone, 
                        0,
                        0,
                        cut_list[0].1 - cut_list[0].0 / 2,
                        185
                    );
                    img_sub1_sub1.to_image().save("img_sub1_sub1.bmp").unwrap();
                    let img_sub1_sub2 = image::imageops::crop(
                        &mut img_sub1, 
                        cut_list[0].1 - cut_list[0].0 / 2,
                        0,
                        cut_list[0].1,
                        185
                    );
                    img_sub1_sub2.to_image().save("img_sub1_sub2.bmp").unwrap();
                }
            /*
            else if ((cut_list[i].1 - cut_list[i].0) as f32) > (3.0 * 30.0 * (1.0-param_fuse)) && 
                    ((cut_list[i].1 - cut_list[i].0) as f32) < (3.0 * 30.0 * (1.0+param_fuse))
                {
                    println!("three char");
                }
            else if ((cut_list[i].1 - cut_list[i].0) as f32) > (4.0 * 30.0 * (1.0-param_fuse)) && 
                    ((cut_list[i].1 - cut_list[i].0) as f32) < (4.0 * 30.0 * (1.0+param_fuse))
                {
                    println!("four char");
                }
             */
        } else if cut_list_size == 2 {
            println!("cut to {} x images", cut_list_size);
            let img_sub2 = image::imageops::crop(
                        &mut img_luma, 
                        cut_list[1].0,
                        0,
                        cut_list[1].1 - cut_list[1].0,
                        185
                    );
            img_sub2.to_image().save("img_sub2.bmp").unwrap();
        } else if cut_list_size == 3 {
            println!("cut to {} x images", cut_list_size);
            let img_sub3 = image::imageops::crop(
                        &mut img_luma, 
                        cut_list[2].0,
                        0,
                        cut_list[2].1 - cut_list[2].0,
                        185
                    );
            img_sub3.to_image().save("img_sub3.bmp").unwrap();
        } else if cut_list_size == 4 {
            println!("cut to {} x images", cut_list_size);
            let img_sub4 = image::imageops::crop(
                        &mut img_luma, 
                        cut_list[3].0,
                        0,
                        cut_list[3].1 - cut_list[3].0,
                        185
                    );
            img_sub4.to_image().save("img_sub4.bmp").unwrap();
        } else {
            println!("Err because subimg > 4");
        }



    
/*
    let mut histo_x = Histogram::with_buckets(w as u64, Some(0));
    for i in 0..w as usize {
        histo_x.add(black_x[i] as f64);
    }
//    for bucket in histo_x.buckets() {
//        do_stuff(bucket.start(), bucket.end(), bucket.count());
//    }
    println!("{}", histo_x);
 */
/*
    let dim = img_luma.dimensions();

    let content = img_luma.into_vec();
    let result = tesseract::Tesseract::new(
        Some("/usr/local/share/tessdata/"), 
        Some("eng"))
        .unwrap()
        .set_image(&fn_luma)
        //.set_image_from_mem(&content)
        .unwrap()
        .set_variable("tessedit_char_whitelist", "0123456789abcdefghijklmnopqrstuvwxyz")
        .unwrap()
/* 
        .set_frame(
            &content,
            dim.0 as i32,
            dim.1 as i32,
            1,
            1 * dim.0 as i32,
        )
        .unwrap()
*/      
        .get_text();
    println!("{}", result.unwrap().trim_end_matches('\n'));
/*
    result.ocr_from_frame(
        &content,
        dim.0 as i32,
        dim.1 as i32,
        1,
        1 * dim.0 as i32,
        "eng"
    );
 */
 */

}

fn rgb_filter_blue_ge_2xgreen (rgb_1: u8, rgb_2: u8, rgb_3: u8, filter_param: f32) -> bool {
    if rgb_1 as f32 >= filter_param * rgb_2 as f32 && 
       rgb_3 as f32 >= filter_param * rgb_2 as f32
    {
        true
    } else {
        false
    }
}