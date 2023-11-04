use sqlmicro_execution::executor::ExecutionResponse;
use tabled::builder::Builder;

pub fn display_response(res: ExecutionResponse) {
    match res {
        ExecutionResponse::Select(rows) => {
            let mut builder = Builder::default();

            let row = rows.get(0).expect("Table has no data");

            let columns: Vec<String> = row
                .columns()
                .iter()
                .map(|col| col.name.to_string())
                .collect();

            builder.set_columns(&columns);

            for row in rows.into_iter() {
                builder.add_record(columns.iter().map(|col| row.get(col)));
            }

            println!("{}", builder.build());
        }
        _ => println!("{res}"),
    }
}
