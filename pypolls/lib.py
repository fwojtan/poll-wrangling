import polars as pl

def get_polls():
    pass
    # Download CSV of historical polls
    # Transform data into desired format
    # Scrape webpage for most recent polls
    # Transform scraped data

def get_election_csv():
    pass

def extract_election_csv():
    df = pl.read_csv("president_polls_2.csv")

    # Exclude older polls, first parsing datetime
    df = df.replace_column(df.get_column_index("start_date"), df["start_date"].str.to_datetime("%m/%d/%y"))
    df = df.replace_column(df.get_column_index("end_date"), df["end_date"].str.to_datetime("%m/%d/%y"))
    df = df.filter(pl.col("start_date") > pl.datetime(2024, 6, 1)) # TODO 30 day lookback or similar # TODO we'll actually want to slide this as a window to produce a change graph
    
    # (Maybe optional) Exclude polls that aren't LV or V
    # df = df.filter(pl.col("population") == "lv")

    # Count the number of candidates each question is recording a value for
    counts = df["question_id"].value_counts()
    counts = counts.rename({"count": "n_candidates_asked"})
    df = df.join(counts, "question_id",)

    # Join Kamala/Trump data on question id
    dems = df.filter(pl.col("party") == "DEM")
    reps = df.filter(pl.col("party") == "REP")
    df = dems.join(reps, "question_id", "inner")
    df = df.rename({"candidate_name": "dem_candidate", "pct": "dem_pct", "pct_right": "rep_pct", "candidate_name_right": "rep_candidate"})

    # Project away unnecessary columns
    df = df.select(["question_id", "pollster", "state", "start_date", "end_date", "sample_size", "methodology", "population", "dem_candidate", "dem_pct", "rep_candidate", "rep_pct", "n_candidates_asked"])
    with pl.Config(tbl_cols=-1):
        print(df)