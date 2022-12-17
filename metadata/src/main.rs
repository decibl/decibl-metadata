use metadata::engine;

fn main() {
    println!("Hello, world!");
    engine::cringeit();
    engine::create_all_files();
    let exConfig = engine::Config {
        soundFilesPath: String::from("C:\\Users\\james\\Documents\\GitHub\\cringeit\\metadata\\src\\test"),
    };

    engine::write_whole_config(exConfig);
    



}