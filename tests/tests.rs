use tinybrain;

#[test]
fn test_helloworld() {
  let result = tinybrain::process(
    "
    ++++++++[>++++[>++>+++>+++>+<<<<-]
    >+>+>->>+[<]<-]>>.>---.+++++++..++
    +.>>.<-.<.+++.------.--------.>>+.
    "
    .to_owned(),
  );
  assert_eq!(result, "Hello World!".as_bytes().to_vec());
}

#[test]
#[should_panic]
fn test_wrong_instructions() {
  tinybrain::process("abc".to_owned());
}
