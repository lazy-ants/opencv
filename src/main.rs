extern crate opencv;

use opencv::core as cv;
use opencv::highgui;
use opencv::imgproc;
use opencv::types::{VectorOfint, VectorOfuchar};

fn main() {
    let mut mat = highgui::imread("/Users/dmitriybelyaev/Desktop/input.jpg", highgui::IMREAD_UNCHANGED).unwrap();
    let buffer = VectorOfint::new();
    let mut dest = cv::Mat::new().unwrap();
    let size = cv::Size { width: 1, height: 1 };
    imgproc::gaussian_blur(&mat, &mut dest, size, 0.0, 0.0, imgproc::BORDER_DEFAULT);
    let name = format!("/Users/dmitriybelyaev/Desktop/result/txt.jpg");
    println!("{:?}", highgui::imwrite(&name, &dest, &buffer));            
}
