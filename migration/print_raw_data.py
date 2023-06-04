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

cursorObj = con.cursor()

print("+{:<}+{:<}+{:<}+{:<}+{:<}".format(
    "-"*7, "-"*7, "-"*7, "-"*7, "-"*20))
print("+{:^7}+{:^7}+{:^7}+{:^7}+{:^7}".format(
    "Year", "Sex", "AgeCode", "N", "Cause"))
print("+{:<}+{:<}+{:<}+{:<}+{:<}".format(
    "-"*7, "-"*7, "-"*7, "-"*7, "-"*20))
for i in cursorObj.execute("SELECT * from dead"):
    print("| {:<5} | {:<5} | {:<5} | {:<5} | {:<5}".format(
        i[0], i[2], i[3], i[4], i[1]))
    print("+{:<}+{:<}+{:<}+{:<}+{:<}".format(
        "-"*7, "-"*7, "-"*7, "-"*7, "-"*20))

con.close()
