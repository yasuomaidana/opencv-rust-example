use opencv::core::MatTraitConst;
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

fn main() {
    let img = imread("./images/cat.jpeg", IMREAD_COLOR).expect("Image not found");
    if img.empty() {
        println!("Error loading image");
        return;
    }

    imshow("Image", &img).expect("Failed to show image");
    wait_key(0).expect("Failed to wait for key");
}
