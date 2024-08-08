use serde::Deserialize;
use std::{fs::File, io::Write};

#[derive(Debug, Deserialize)]
struct Joke {
    date: String,
    amount: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Resp {
    data: Vec<Joke>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let year = "2020";
    let resp: Resp = reqwest::blocking::get(format!(
        "http://35.77.82.139:3000/transactions/?fromDate={year}-01-01&toDate={year}-01-05",
    ))
    .expect("get request")
    .json()
    .expect("matching JSON");

    let first_line = "date,amount,description".to_string();
    let text: String = resp.data.into_iter().fold(first_line, |acc, x| {
        let mut date_iter = x.date.split('-');
        let day = date_iter
            .next()
            .expect("provided day")
            .parse::<u8>()
            .expect("day is a number");
        let month = date_iter
            .next()
            .expect("provided month")
            .parse::<u8>()
            .expect("month is a number")
            + 1; // zero indexed month!

        acc + &format!(
            "\n{year}-{month:02}-{day:02},{},'{}'",
            x.amount.replace(",", ""),
            x.description
        )
    });

    let mut file = File::create("output.csv")?;
    file.write_all(text.as_bytes())?;

    Ok(())
}
