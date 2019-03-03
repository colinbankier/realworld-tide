extern crate realworld_tide;

fn main() {
    let app = realworld_tide::application();
    app.serve();
}
