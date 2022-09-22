#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
#![allow(unused_variables)]

use std::env;
//use substring::Substring;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, RgbaImage, Rgba, GrayImage};
//use imageproc::contrast::threshold;


fn main() {
    // input file name and parameters
    struct arg_input {
        path: std::path::PathBuf,
        //param: String,
    }
    let file_name = env::args().nth(1).expect("no file name");
    //let param     = env::args().nth(2).expect("no param");
    let args = arg_input {
        path: std::path::PathBuf::from(file_name),
        //param: param,
    };
    //let filter_param = args.param.parse::<f32>().unwrap();
    //println!("{:#?} {:#?}", args.path, args.param.parse::<f32>().unwrap());
    // base file name
    let fname_base: String = args.path.file_name().unwrap().to_str().unwrap().into();
    let fn_base = fname_base.get(..11);
    //println!("{:#?}", fn_base.unwrap());

    // start images business
    let img = image::open(args.path).unwrap();
    
    // copyimg_new from img
    let mut img_new = img.clone();
    // save image toimg_new
    let fn_new = format!("{base}orig{ext}", 
            base = fn_base.unwrap(), 
            ext = ".jpg");
    //println!("{:?}",fn_new);
    img_new.save(&fn_new).unwrap();
    

}
