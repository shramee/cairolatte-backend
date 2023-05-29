// use serde::{Deserialize, Serialize};
use serde_json::Value;
mod json_engine;
// use std::fs::File;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

use hanji::run_printer;
use hanji::utils::get_cairo_files_in_path;
fn main() {
    let out_path = PathBuf::from("out");

    let repos = vec![
        "greged93/2wrds_cntrcts",
    ];

    create_dir_all(&out_path).unwrap();
    let mut doc_file_path = out_path.clone();
    doc_file_path.push("functions-source");
    doc_file_path.set_extension("js");
    create_dir_all(&doc_file_path.parent().unwrap()).unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&doc_file_path.into_os_string().to_str().unwrap())
        .unwrap();
    for (i, repo) in repos.iter().enumerate() {
        // let mut json_path = out_path.clone();
        println!("{i} Processing {repo}");
        file = process_files(format!("cairo-repos/{repo}"), repo, file);
    }

    // Cairo language spec
    // cairo_lang_syntax_codegen::cairo_spec::get_spec
}

fn process_files(path: String, repo: &str, mut file: File) -> File {
    let in_path = PathBuf::from(&path);

    println!("{in_path:?}");

    // remove_dir_all(&out_path).unwrap();
    let cairo_files = get_cairo_files_in_path(&in_path);

    let mut functions = String::from("\n// ");

    functions.push_str(repo);
    functions.push_str("\n");

    for cairo_file in cairo_files.iter() {
        println!("Processing {cairo_file:?}");
        functions.push_str(&run_printer(
            cairo_file.to_str().unwrap(),
            hanji::MarkdownEngine::new(),
        ));
    }

    file.write_all(functions.as_bytes()).unwrap();

    file
}
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
fn untyped_example(data: &str) -> Value {
    serde_json::from_str(data).unwrap()
}
fn main2() {
    let data = r#"
	{
		"name": "John Doe",
		"age": 43,
		"phones": [
			"+44 1234567",
			"+44 2345678"
		]
	}"#;
    let v = untyped_example(data);
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
}
