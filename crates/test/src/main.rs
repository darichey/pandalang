#![feature(iterator_try_collect)]

extern crate glob;

use clap::Parser;
use glob::glob;
use libtest_mimic::Trial;
use similar_asserts::SimpleDiff;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug, Clone, Default)]
struct MyArguments {
    #[clap(flatten)]
    inner: libtest_mimic::Arguments,
    /// When true, .expected files will be re-recorded
    #[arg(short, long)]
    record: bool,
}

fn main() {
    let args = MyArguments::parse();
    let tests = get_tests(args.record);
    libtest_mimic::run(&args.inner, tests).exit();
}

fn get_tests(record: bool) -> Vec<Trial> {
    get_parse_tests(record)
}

fn get_eval_tests() -> Vec<Trial> {
    todo!()
}

fn get_parse_tests(record: bool) -> Vec<Trial> {
    get_input_sources("inputs/parse/exprs/**/*.panda")
        .into_iter()
        .map(|(path, src)| {
            Trial::test(path.clone(), move || {
                let actual = format!("{:#?}", pandalang::parser::parse_expr(&src));
                let expected_path: PathBuf = format!("{}.expected", path).into();
                if record {
                    fs::write(expected_path, actual).unwrap();
                    Ok(())
                } else if expected_path.exists() {
                    let expected = fs::read_to_string(expected_path).unwrap();
                    if expected == actual {
                        Ok(())
                    } else {
                        let diff = SimpleDiff::from_str(&expected, &actual, "expected", "actual");
                        Err(diff.into())
                    }
                } else {
                    Err("Couldn't find .expected file. Did you mean to --record it?".into())
                }
            })
        })
        .collect()
}

fn get_type_check_tests() -> Vec<Trial> {
    todo!()
}

fn get_input_sources(pattern: &str) -> Vec<(String, String)> {
    glob(pattern)
        .unwrap()
        .map(|path| {
            let path = path.unwrap();
            let src = fs::read_to_string(&path).unwrap();
            let path = path.into_os_string().into_string().unwrap();
            (path, src)
        })
        .collect()
}
