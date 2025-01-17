//! Example of how to use the raw bindings
//!

extern crate failure;
extern crate openslide;

use openslide::{bindings, utils};
use std::path::Path;
use failure::Error;

fn basic_usage(
    filename: &str
) -> Result<(), Error> {

    unsafe {
        let vendor = bindings::detect_vendor(filename)?;
        println!("Vendor: {}", vendor);

        let osr = bindings::open(filename)?;

        let levels = bindings::get_level_count(osr)?;
        println!("Slide has {} levels", levels);

        let (height, width) = bindings::get_level0_dimensions(osr)?;
        println!("Slide has dimension {} x {} at level 0", height, width);

        let level = 0;
        let (height, width) = bindings::get_level_dimensions(osr, level)?;
        println!("Slide has dimension {} x {} at level {}", height, width, level);

        let factor = bindings::get_level_downsample(osr, level)?;
        println!("Slide at level {} is downsampled with factor {}", level, factor);

        let downsample_factor = 5.6;
        let level = bindings::get_best_level_for_downsample(osr, downsample_factor)?;
        println!("Best level for downsample factor {} is {}", downsample_factor, level);

        let x = 1000;
        let y = 1500;
        let level = 0;
        let h = 512;
        let w = 512;
        let word_repr = utils::WordRepresentation::BigEndian;
        let buffer = bindings::read_region(osr, x, y, level, w, h)?;
        let im = utils::decode_buffer(&buffer, h as u32, w as u32, word_repr)?;
        im.save(Path::new("/tmp/wsi_region_1.png"))?;
        println!("Region is written");

        // Test error
        //let factor = bindings::get_level_downsample(osr, 2)?;
        //println!("{:?}", bindings::get_error(osr));

        bindings::close(osr);
    }

    Ok(())
}

fn properties(
    filename: &str
) -> Result<(), Error> {
    let osr = bindings::open(filename)?;

    println!("Slide in {} has the following properties:", filename);
    println!("{0:<40} {1}", "Property key", "Property value");
    unsafe {
        for name in bindings::get_property_names(osr)? {
            println!("{0:<40} {1}", name, bindings::get_property_value(osr, &name)?);
        }

        bindings::close(osr);
    }
    Ok(())
}


fn associated_images(
    filename: &str
) -> Result<(), Error> {
    let osr = bindings::open(filename)?;

    println!("Slide in {} has the following associated images:", filename);
    unsafe {
        for name in bindings::get_associated_image_names(osr)? {
            println!("{0:<40}", name);
            let (width, height) = bindings::get_associated_image_dimensions(osr, &name)?;
            println!("Associated image '{}' has dimension {} x {}", name, width, height);
            let word_repr = utils::WordRepresentation::BigEndian;
            let buffer = bindings::read_associated_image(osr, &name)?;
            let im = utils::decode_buffer(&buffer, height as u32, width as u32, word_repr)?;
            let dist_path = format!("/tmp/associated_image_{}.png", name);
            im.save(Path::new(&dist_path))?;
            println!("Associated image '{}' is written", name);
        }

        bindings::close(osr);
    }
    Ok(())
}

fn main() {
    let filename = "assets/CMU-1-Small-Region.svs";
    println!("Analyzing {}", filename);

    match basic_usage(filename) {
        Ok(_) => println!("Basic usage functions are working okay"),
        Err(msg) => {
            println!("Basic usage functions not working");
            println!("{}", msg);
        },
    }

    match properties(filename) {
        Ok(_) => println!("Property functions are working okay"),
        Err(msg) => {
            println!("Property functions not working");
            println!("{}", msg);
        },
    }

    match associated_images(filename) {
        Ok(_) => println!("Associated image functions are working okay"),
        Err(msg) => {
            println!("Associated image functions not working");
            println!("{}", msg);
        },
    }

    println!("Example program is terminated");
}
