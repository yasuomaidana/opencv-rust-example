use opencv::core::{Mat, MatTraitConst, CV_32F, CV_8U, min_max_loc, BORDER_REFLECT_101};

use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::imgproc::{cvt_color, COLOR_BGR2GRAY, sobel};

fn main() {
    let img = imread("./images/cat.jpeg", IMREAD_COLOR)
        .expect("Image not found");

    // Convert the image to grayscale
    let mut grey: Mat = Mat::default();
    cvt_color(&img, &mut grey, COLOR_BGR2GRAY,1).unwrap();

    // Apply Sobel operator to find gradients in the x-direction
    let mut sobelx: Mat = Mat::default();
    sobel(&grey, &mut sobelx, CV_32F, 1, 0, 3, 1.0, 0.0, BORDER_REFLECT_101).unwrap();

    // Find the minimum and maximum values in the Sobel gradient map
    let mut min_val: f64 = 0.0;
    let mut max_val: f64 = 0.0;
    min_max_loc(&sobelx, Some(&mut min_val), Some(&mut max_val), None, None, &Mat::default()).unwrap();


    // Scale the Sobel gradient map to [0, 255] for visualization
    let mut draw: Mat = Mat::default();
    sobelx.convert_to(&mut draw, CV_8U,
                      255.0 / (max_val - min_val),
                      -min_val * 255.0 / (max_val - min_val)).unwrap();

    // Display the processed image
    imshow("Sobel Edge Detection", &draw)
        .expect("Failed to show image");

    wait_key(0)
        .expect("Failed to wait for key");
}