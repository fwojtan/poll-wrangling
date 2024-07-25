use polars::frame::DataFrame;

/// question_id should be parseable as i64, as should sample size
/// state will be checked as a valid US state
/// date_range should be in the format %d-%d %b
#[derive(Debug)]
pub struct Scraped538Poll {
    pub question_id: String,
    pub pollster: String,
    pub state: Option<String>,
    pub date_range: String,
    pub sample_size: String,
    pub population: String,
    /// (candidate_name: 'Biden', percent: '42%')
    pub answers: Vec<(String, String)>,
}

pub struct PollFrameBuilder {
    question_ids: Vec<i64>,
    pollsters: Vec<String>,
    states: Vec<String>,
    start_dates: Vec<String>,
    end_dates: Vec<String>,
    sample_sizes: Vec<i64>,
    methodologies: Vec<String>,
    populations: Vec<String>,
    dem_candidates: Vec<String>,
    dem_pcts: Vec<f64>,
    rep_candidates: Vec<String>,
    rep_pcts: Vec<f64>,
    n_candidates: Vec<u32>,
}

impl PollFrameBuilder {
    pub fn new() -> Self {
        PollFrameBuilder {
            question_ids: vec![],
            pollsters: vec![],
            states: vec![],
            start_dates: vec![],
            end_dates: vec![],
            sample_sizes: vec![],
            methodologies: vec![],
            populations: vec![],
            dem_candidates: vec![],
            dem_pcts: vec![],
            rep_candidates: vec![],
            rep_pcts: vec![],
            n_candidates: vec![],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PollFrameBuilder {
            question_ids: Vec::with_capacity(capacity),
            pollsters: Vec::with_capacity(capacity),
            states: Vec::with_capacity(capacity),
            start_dates: Vec::with_capacity(capacity),
            end_dates: Vec::with_capacity(capacity),
            sample_sizes: Vec::with_capacity(capacity),
            methodologies: Vec::with_capacity(capacity),
            populations: Vec::with_capacity(capacity),
            dem_candidates: Vec::with_capacity(capacity),
            dem_pcts: Vec::with_capacity(capacity),
            rep_candidates: Vec::with_capacity(capacity),
            rep_pcts: Vec::with_capacity(capacity),
            n_candidates: Vec::with_capacity(capacity),
        }
    }

    /// Returns an error for a variety of parse failures
    /// Parses numeric values and datetimes from expected formats
    /// Methodology is "Unknown"
    pub fn add_538_poll(&mut self, poll: Scraped538Poll) -> Result<(), String> {
        todo!();
        Ok(())
    }

    pub fn build(self) -> DataFrame {
        todo!()
    }
}
