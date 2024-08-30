use opencv::core::{MatTraitConst, Vec3b};
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

fn main() {

    let img = imread("./images/cat.jpeg", IMREAD_COLOR).expect("Image not found");
    let a = img.at_2d::<Vec3b>(220, 150).unwrap();
    println!("Pixel value at (220, 150): {:?}", a);
    imshow("Original image", &img).expect("Failed to show image");
    wait_key(0).expect("Failed to wait for key");
}
