pub fn print_file(file_name: String) {
    let contents =
        std::fs::read_to_string(file_name.clone()).expect("Something went wrong reading the file");
    println!(
        "File Name -> {}\nContent ->\n{}",
        file_name.clone(),
        contents
    );
}
