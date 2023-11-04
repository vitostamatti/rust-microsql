pub mod error;
pub mod response;
use error::SqlMicroError;
use miette::GraphicalReportHandler;
use response::display_response;
use rustyline::{error::ReadlineError, DefaultEditor, Result};
use sqlmicro_execution::executor::{ExecutionResponse, Executor};
use sqlmicro_parser::{parse::Parse, query::SqlQuery};
const HISTORY_FILE: &str = "./history.txt";

fn parse_and_run<'a>(
    exec: &'a mut Executor,
    query: &'a str,
) -> std::result::Result<ExecutionResponse<'a>, SqlMicroError<'a>> {
    let query: SqlQuery = SqlQuery::parse_format_error(query)?;
    let res = exec.run(query)?;
    Ok(res)
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history found.");
    }

    let mut exec = Executor::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

                let query: &str = line.as_ref();
                let res = parse_and_run(&mut exec, query);

                match res {
                    Ok(res) => display_response(res),
                    Err(e) => {
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .with_cause_chain()
                            .with_context_lines(10)
                            .render_report(&mut s, &e)
                            .unwrap();
                        println!("{s}");
                    }
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE)
}
