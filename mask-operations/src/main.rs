use opencv::highgui::named_window;

fn main() {
    named_window("window", 0).unwrap();

    loop {
        if opencv::highgui::wait_key(10).unwrap() > 0 {
            break;
        }
    }
}
