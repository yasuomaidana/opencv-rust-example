use opencv::core::{Mat, MatTraitConst, Size, Vec3b, BORDER_DEFAULT};
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

    wait_key(0).expect("Failed to wait for key");
}
