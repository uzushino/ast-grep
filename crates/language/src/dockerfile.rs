#![cfg(test)]
use ast_grep_core::{source::TSParseError, Matcher, Pattern};

use super::*;

fn test_match(s1: &str, s2: &str) {
  let pattern = Pattern::str(s1, Dockerfile);
  let cand = Dockerfile.ast_grep(s2);

  dbg!(&pattern);
  dbg!(&cand.root().to_sexp());

  assert!(
    pattern.find_node(cand.root()).is_some(),
    "goal: {:?}, candidate: {}",
    pattern,
    cand.root().to_sexp(),
  );
}

#[test]
fn test_docker_str() {
  test_match("FROM $A:1.0", "FROM hoge/mysql:1.0");
}