use opencv::core::{hconcat, Mat, Vector};
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::imgproc::filter_2d;

fn main() {
    let kernel_values: Vec<f32> = vec![
        0.0, -1.0, 0.0,
        -1.0, 5.0, -1.0,
        0.0, -1.0, 0.0,
    ];

    // Create a Mat object with the kernel values
    let kernel = Mat::from_slice(&kernel_values).unwrap();
    let img = imread("./images/cat.jpeg", IMREAD_COLOR).expect("Image not found");
    let mut output = Mat::default();
    filter_2d(&img, &mut output, -1, &kernel, Default::default(), Default::default(), 0).unwrap();
    let mut comparison = Mat::default();


    let mut images:Vector<Mat> = Vector::new();
    images.push(img);
    images.push(output);

    hconcat(&images, &mut comparison).expect("Failed to concatenate images");

    imshow("Image", &comparison).expect("Failed to show image");
    wait_key(0).expect("Failed to wait for key");
}
