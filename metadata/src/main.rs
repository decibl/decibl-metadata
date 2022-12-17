use metadata::engine;

fn main() {
    println!("Hello, world!");
    engine::cringeit();
    engine::create_all_files();
    engine::write_soundfiles_path("C:\\Users\\decib\\Desktop\\soundfiles");
    let path = engine::get_soundfiles_path();

    println!("The path is: {}", path);
}