#[cfg(test)]
mod smoke {
    use anyhow::Result;
    use assert_cmd::Command;

    const DB_PATH: &str = "../../.data/test.sqlite3";
    const SQL_FILE: &str = "test.sql";
    const TEST_TOPIC: &str = "test-sql";

    #[ignore]
    #[tokio::test]
    async fn cli() -> Result<()> {
        let mut cmd = Command::cargo_bin("sql2fluvio").expect("Binary exists");

        cmd.arg(DB_PATH).arg(SQL_FILE).arg(TEST_TOPIC);

        let assert = cmd.assert();

        let output = assert.get_output();
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{stdout}");
        assert.success();

        Ok(())
    }
}
