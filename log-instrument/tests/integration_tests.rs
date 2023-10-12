const ONE: &str = "../target/debug/examples/one";
const TWO: &str = "../target/debug/examples/two";
const THREE: &str = "../target/debug/examples/three";
const FOUR: &str = "../target/debug/examples/four";
const FIVE: &str = "../target/debug/examples/five";
const SIX: &str = "../target/debug/examples/six";

const TIMESTAMP_RANGE: std::ops::Range<usize> = 1..20;

/// Match stderr to expected skipping bytes containing the timestamp.
fn check(actual: &[u8], expected: &[u8]) {
    assert_eq!(actual.len(), expected.len());
    let mut j = 0;
    for i in 0..actual.len() {
        if actual[i] == b'\n' {
            j = 0;
        }
        if TIMESTAMP_RANGE.contains(&j) {
            continue;
        }
        assert_eq!(actual[i], expected[i]);
        j += 1;
    }
}

#[test]
fn one() {
    let output = std::process::Command::new(ONE).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    let expected_stderr = b"\
        [2023-10-12T16:29:00Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:29:00Z DEBUG one] cmp: true\n\
        [2023-10-12T16:29:00Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:29:00Z INFO  one] 4\n\
        [2023-10-12T16:29:00Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:29:00Z DEBUG one] cmp: false\n\
        [2023-10-12T16:29:00Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:29:00Z INFO  one] 6\n\
        [2023-10-12T16:29:00Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:29:00Z DEBUG one] cmp: false\n\
        [2023-10-12T16:29:00Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:29:00Z INFO  one] 7\n\
    ";
    check(&output.stderr, expected_stderr);
}

#[test]
fn two() {
    let output = std::process::Command::new(TWO).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    let expected_stderr = b"\
        [2023-10-12T16:29:30Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:29:30Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:29:30Z INFO  two] None\n\
        [2023-10-12T16:29:30Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:29:30Z DEBUG two] [\"a\", \"b\"]\n\
        [2023-10-12T16:29:30Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:29:30Z INFO  two] Some([\"a\", \"b\"])\n\
    ";
    check(&output.stderr, expected_stderr);
}

#[test]
fn three() {
    let output = std::process::Command::new(THREE).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    let expected_stderr = b"\
        [2023-10-12T16:30:04Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:30:04Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:30:04Z INFO  three] None\n\
        [2023-10-12T16:30:04Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:30:04Z DEBUG three] [\"a\", \"b\"]\n\
        [2023-10-12T16:30:04Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:30:04Z INFO  three] Some([\"a\", \"b\"])\n\
    ";
    check(&output.stderr, expected_stderr);
}

#[test]
fn four() {
    let output = std::process::Command::new(FOUR).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    let expected_stderr = b"\
        [2023-10-12T16:30:37Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:30:37Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:30:37Z INFO  four] None\n\
        [2023-10-12T16:30:37Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:30:37Z DEBUG four] [\"a\", \"b\"]\n\
        [2023-10-12T16:30:37Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:30:37Z INFO  four] Some([\"a\", \"b\"])\n\
    ";
    check(&output.stderr, expected_stderr);
}

#[test]
fn five() {
    let output = std::process::Command::new(FIVE).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    println!("{:?}", std::str::from_utf8(&output.stderr).unwrap());
    let expected_stderr = b"\
        [2023-10-12T16:31:12Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:31:12Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:31:12Z INFO  five] None\n\
        [2023-10-12T16:31:12Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:31:12Z DEBUG five] [\"a\", \"b\"]\n\
        [2023-10-12T16:31:12Z DEBUG five] 23\n\
        [2023-10-12T16:31:12Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:31:12Z INFO  five] Some([\"a\", \"b\"])\n\
    ";
    check(&output.stderr, expected_stderr);
}

#[test]
fn six() {
    let output = std::process::Command::new(SIX).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    println!("{:?}", std::str::from_utf8(&output.stderr).unwrap());
    let expected_stderr = b"\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:31:54Z DEBUG six] cmp: true\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:31:54Z INFO  six] 4\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:31:54Z DEBUG six] cmp: false\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)::one>>two\n\
        [2023-10-12T16:31:54Z DEBUG six] res: 0\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)::one<<two\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:31:54Z INFO  six] 0\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)>>one\n\
        [2023-10-12T16:31:54Z DEBUG six] cmp: false\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)::one>>two\n\
        [2023-10-12T16:31:54Z DEBUG six] res: 1\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)::one<<two\n\
        [2023-10-12T16:31:54Z TRACE log_instrument] ThreadId(1)<<one\n\
        [2023-10-12T16:31:54Z INFO  six] 1\n\
    ";
    check(&output.stderr, expected_stderr);
}
