import pandas as pd
import sqlite3
import pathlib
from dotenv import load_dotenv
import os

current_path = pathlib.Path(__file__).parent.resolve()
root_path = f"{current_path}/.."

env_path = f"{root_path}/.env"
load_dotenv(env_path)

data_base_url = os.getenv("DATABASE_URL")

con = sqlite3.connect(f"{root_path}/{data_base_url[9:]}")

df = pd.read_sql_query("SELECT * FROM dead", con)

group = df.groupby("year")

for year, data in group:
    (pd.DataFrame(data)
        .head(400)
        .to_html(f"output/{year}.html", index=False))
