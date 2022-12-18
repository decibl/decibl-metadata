use metadata::engine;

fn main() {
    println!("Hello, world!");
    engine::cringeit();
    engine::create_all_files();
    let exConfig = engine::Config {
        soundFilesPath: String::from("C:\\Users\\james\\Documents\\GitHub\\cringeit\\metadata\\src\\test"),
    };

    engine::write_whole_config(exConfig);
    let config = engine::write_config_var("bruh", "bruhf");

    let contents = engine::get_config_as_str();

    println!("The contents are: {}", contents);

    let song_table_str = engine::compile_table(&engine::SONGS);
    println!("The song table is: {}", song_table_str);




}