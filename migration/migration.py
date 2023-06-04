import pandas as pd
import sqlite3
import pathlib
from dotenv import load_dotenv
import os

from cause_list import (
    cause_list_before_96,
    cause_list_between_97_108,
    cause_list_after_109
)

data = pd.DataFrame()
current_path = pathlib.Path(__file__).parent.resolve()
root_path = f"{current_path}/.."

for i in range(81, 97):
    df = pd.read_csv(f"{current_path}/data/dead{i}.csv")
    df.drop(columns="county", inplace=True)
    df['cause'] = df['cause'].replace(list(range(1, 36)), cause_list_before_96)
    data = pd.concat([data, df], axis=0, ignore_index=True)


for i in range(97, 109):
    df = pd.read_csv(f"{current_path}/data/dead{i}.csv")
    df.drop(columns="county", inplace=True)
    df['cause'] = df['cause'].replace(
        list(range(1, 42)), cause_list_between_97_108)
    data = pd.concat([data, df], axis=0, ignore_index=True)


for i in range(109, 111):
    df = pd.read_csv(f"{current_path}/data/dead{i}.csv")
    df.drop(columns="county", inplace=True)
    df['cause'] = df['cause'].replace(list(range(1, 43)), cause_list_after_109)
    data = pd.concat([data, df], axis=0, ignore_index=True)

# print(data.groupby('year').count())

env_path = f"{root_path}/.env"
load_dotenv(env_path)

data_base_url = os.getenv("DATABASE_URL")
# slice off `sqlite://`
con = sqlite3.connect(f"{root_path}/{data_base_url[9:]}")
data.to_sql("dead", con=con, if_exists='append')
