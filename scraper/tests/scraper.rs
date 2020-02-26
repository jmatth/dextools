use assert_cmd::Command;
use tempfile::NamedTempFile;

fn verify_output(input_path: &str, expected_path: &str) {
	let output = NamedTempFile::new().unwrap();
	let mut scrape_cmd = Command::cargo_bin("dextools-scraper").unwrap();
	scrape_cmd
		.arg("-i")
		.arg(input_path)
		.arg("-o")
		.arg(output.path())
		.assert()
		.success();
	let mut diff_cmd = Command::new("diff");
	diff_cmd
		.arg("-u")
		.arg("--color=always")
		.arg(expected_path)
		.arg(output.path())
		.assert()
		.success();
}

#[test]
fn dreamation_test() {
	verify_output(
		"./tests/fixtures/d2019complete.html",
		"./tests/fixtures/d2019complete.json",
	);
}

#[test]
fn dexcon_test() {
	verify_output(
		"./tests/fixtures/d22complete.html",
		"./tests/fixtures/d22complete.json",
	);
}

#[test]
fn metatopia_test() {
	verify_output(
		"./tests/fixtures/m2019complete.html",
		"./tests/fixtures/m2019complete.json",
	);
}
