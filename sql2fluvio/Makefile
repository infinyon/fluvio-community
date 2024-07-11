build:
	cargo build --release

sql2fluvio=../../target/release/sql2fluvio
db_path=../../schema/test.sqlite3
sql_path=test.sql

run:
	$(sql2fluvio) $(db_path) $(sql_path) data
