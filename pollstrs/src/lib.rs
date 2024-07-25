mod boilerplate;
mod poll_frame;

use std::{fs::File, io::Write};

use pyo3::prelude::*;
use reqwest::blocking::get;

use crate::poll_frame::{PollFrameBuilder, Scraped538Poll};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn scrape_latest_538() -> PyResult<String> {
    let response =
        get("https://projects.fivethirtyeight.com/polls/president-general/2024/national/");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);

    let html_selector = scraper::Selector::parse(".visible-row").unwrap();
    let html_rows = document.select(&html_selector);

    let mut builder = PollFrameBuilder::with_capacity(100);

    for row in html_rows {
        let question_id = row.attr("data-id").unwrap_or("999999").to_string();
        let pollster = row
            .select(&scraper::Selector::parse(".pollster-name").unwrap())
            .next()
            .expect("No pollster name in visible row")
            .inner_html();

        // TODO state becomes an argument when this is refactored out.. For now national= None
        let state: Option<String> = None;

        let date_range = row
            .select(&scraper::Selector::parse(".date-wrapper").unwrap())
            .next()
            .expect("No date in visible row")
            .inner_html();

        let sample_size = row
            .select(&scraper::Selector::parse(".sample").unwrap())
            .next()
            .expect("No sample size in visible row")
            .inner_html();

        // Methodology is unknown

        let population = row
            .select(&scraper::Selector::parse(".sample-type").unwrap())
            .next()
            .expect("No population type in visible row")
            .inner_html()
            .to_lowercase();

        let answers_container = row
            .select(&scraper::Selector::parse(".mobile-answers-container").unwrap())
            .next()
            .expect("No population type in visible row");
        let mut answers = vec![];
        for answer in answers_container.select(&scraper::Selector::parse(".mobile-answer").unwrap())
        {
            let candidate = answer
                .select(&scraper::Selector::parse("p").unwrap())
                .next()
                .expect("No candidate name in given answer")
                .inner_html();
            let pct = answer
                .select(&scraper::Selector::parse("div").unwrap())
                .next()
                .expect("No percentage in given answer")
                .inner_html();
            answers.push((candidate, pct));
        }

        println!("{:?}", Scraped538Poll {
            question_id: question_id.clone(),
            pollster: pollster.clone(),
            state: None,
            date_range: date_range.clone(),
            sample_size: sample_size.clone(),
            population: population.clone(),
            answers: answers.clone(),
        });

        builder.add_538_poll(Scraped538Poll {
            question_id,
            pollster,
            state: None,
            date_range,
            sample_size,
            population,
            answers,
        });
    }
    todo!()
}

// We want to scrape the national page and every state
// Implement random ordering and random-stutter timing
// Ideally this is super slow so we don't get in trouble, but that would also be annoying
// Test once and loop fr

/// A Python module implemented in Rust.
#[pymodule]
fn pollstrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scrape_latest_538, m)?)?;
    Ok(())
}
