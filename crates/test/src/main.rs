extern crate glob;

use clap::Parser;
use glob::glob;
use libtest_mimic::{Failed, Trial};
use pandalang::value::Value;
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
    let parse_tests = get_parse_tests(record);
    let type_check_tests = get_type_check_tests(record);
    let eval_tests = get_eval_tests(record);

    parse_tests
        .chain(type_check_tests)
        .chain(eval_tests)
        .collect()
}

#[allow(dead_code)] // This struct is only used for debug print
#[derive(Debug)]
struct ProgramOutput {
    main_return: Value,
    stdout: String,
}

fn get_eval_tests(record: bool) -> impl Iterator<Item = Trial> {
    get_input_sources("inputs/eval/**/*.panda").map(snapshot_trial(record, |src| {
        let program = pandalang::parser::parse(&src).map_err(|err| err.to_string())?;
        let mut stdout = Vec::new();
        let result =
            pandalang::eval::run_program(program, &mut stdout).map(|main_return| ProgramOutput {
                main_return,
                stdout: String::from_utf8_lossy(&stdout).into_owned(),
            });
        Ok(format!("{:#?}", result))
    }))
}

fn get_parse_tests(record: bool) -> impl Iterator<Item = Trial> {
    let expr_trials = get_input_sources("inputs/parse/exprs/**/*.panda")
        .map(snapshot_trial(record, |src| {
            Ok(format!("{:#?}", pandalang::parser::parse_expr(&src)))
        }));

    let prog_trials = get_input_sources("inputs/parse/progs/**/*.panda")
        .map(snapshot_trial(record, |src| {
            Ok(format!("{:#?}", pandalang::parser::parse(&src)))
        }));

    let type_trials = get_input_sources("inputs/parse/types/**/*.panda")
        .map(snapshot_trial(record, |src| {
            Ok(format!("{:#?}", pandalang::parser::parse_type(&src)))
        }));

    expr_trials.chain(prog_trials).chain(type_trials)
}

fn get_type_check_tests(record: bool) -> impl Iterator<Item = Trial> {
    let expr_trials = get_input_sources("inputs/type_check/exprs/**/*.panda").map(snapshot_trial(
        record,
        |src| {
            let ast = *pandalang::parser::parse_expr(&src).map_err(|err| err.to_string())?;
            Ok(format!(
                "{:#?}",
                pandalang::types::check_expr_to_string(ast)
            ))
        },
    ));

    let prog_trials = get_input_sources("inputs/type_check/progs/**/*.panda").map(snapshot_trial(
        record,
        |src| {
            let program = pandalang::parser::parse(&src).map_err(|err| err.to_string())?;
            Ok(format!(
                "{:#?}",
                pandalang::types::check_prog_to_strings(program)
            ))
        },
    ));

    expr_trials.chain(prog_trials)
}

struct InputSource {
    path: String,
    src: String,
}

fn get_input_sources(pattern: &str) -> impl Iterator<Item = InputSource> {
    glob(pattern).unwrap().map(|path| {
        let path = path.unwrap();
        let src = fs::read_to_string(&path).unwrap();
        let path = path.into_os_string().into_string().unwrap();
        InputSource { path, src }
    })
}

fn snapshot_trial(
    record: bool,
    get_actual: fn(String) -> Result<String, String>,
) -> impl FnMut(InputSource) -> Trial {
    move |InputSource { path, src }| {
        Trial::test(path.clone(), move || {
            let actual = get_actual(src).map_err(Failed::from)?;
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
    }
}
