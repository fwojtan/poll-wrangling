import pollstrs

from pypolls.lib import extract_election_csv

def scrape():
    pollstrs.scrape_latest_538()
    print("hello python")

def tryout():
    extract_election_csv()