use pandoc_types::Pandoc;
use std::process::{Command, Stdio};
use std::io::Write;

fn deser_same(src: &[u8]) {
	let mut p = Command::new("pandoc")
		.args(&["-t", "json", "-f", "markdown"])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("Error spawning pandoc");
	{
		let stdin = p.stdin.as_mut()
    		.expect("Failed to open stdin");
    	stdin.write_all(src)
    		.expect("Failed to write to stdin");
	}
	let output = p.wait_with_output()
		.expect("Output err");
	let json = String::from_utf8_lossy(&output.stdout);
	let pandoc: Pandoc = serde_json::from_str(&json)
		.expect("Error deserializing json");
	let ser = serde_json::to_string(&pandoc)
		.expect("Error serializing");

	let mut cmd = Command::new("pandoc")
		.args(&["-f", "json", "-t", "json"])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("Error spawning pandoc");
	{
		let stdin = cmd.stdin.as_mut()
    		.expect("Failed to open stdin");
    	stdin.write_all(ser.as_bytes())
    		.expect("Failed to write to stdin");
	}
	let ser_output = cmd.wait_with_output()
		.expect("Output err");
	
	let mut cmd = Command::new("pandoc")
		.args(&["-f", "json", "-t", "json"])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("Error spawning pandoc");
	{
		let stdin = cmd.stdin.as_mut()
    		.expect("Failed to open stdin");
    	stdin.write_all(json.as_bytes())
    		.expect("Failed to write to stdin");
	}
	let orig_output = cmd.wait_with_output()
		.expect("Output err");


	assert_eq!(orig_output, ser_output);
}

#[test]
fn deserializing_roundtrip() {
	let t1 = include_bytes!("testsuite.txt");
	let t2 = include_bytes!("tables.txt");
	let t3 = include_bytes!("markdown-reader-more.txt");
	deser_same(t1);
	deser_same(t2);
	deser_same(t3);
}