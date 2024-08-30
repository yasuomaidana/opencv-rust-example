use opencv::core::{min_max_loc, Mat, MatTraitConst, Size, Vec3b, BORDER_DEFAULT, CV_8U};
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::imgproc::{cvt_color, COLOR_BGR2GRAY, gaussian_blur};

fn main() {

    let img = imread("./images/cat.jpeg", IMREAD_COLOR).expect("Image not found");
    let a = img.at_2d::<Vec3b>(220, 150).unwrap();
    println!("Pixel value at (220, 150): {:?}", a);
    imshow("Original image", &img).expect("Failed to show image");

    let mut output = Mat::default();

    cvt_color(&img, &mut output, COLOR_BGR2GRAY,1).unwrap();

    let kernel_size = Size::new(11, 11);
    let mut gaussian_output = Mat::default();

    gaussian_blur(&output,&mut gaussian_output, kernel_size, 25.0, 25.0, BORDER_DEFAULT).unwrap();
    imshow("Gray-Gaussian image", &gaussian_output).unwrap();

    let mut min_val = 0.0;
    let mut max_val = 0.0;
    let mut min_point = Default::default();
    let mut max_point = Default::default();
    let default_mask = Mat::default();

    min_max_loc(&gaussian_output,
                Some(&mut min_val), Some(&mut max_val),
                Some(&mut min_point), Some(&mut max_point),
                &default_mask).unwrap();

    println!("Min value: {}, Max value: {}", min_val, max_val);
    println!("Min point: {:?}, Max point: {:?}", min_point, max_point);

    let mut draw = Mat::default();
    gaussian_output.convert_to(&mut draw, CV_8U, 255.0/(max_val - min_val), -min_val * 255.0/(max_val - min_val)).unwrap();
    imshow("Final Draw", &draw).unwrap();
    wait_key(0).expect("Failed to wait for key");
}
