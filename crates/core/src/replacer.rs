use crate::meta_var::{extract_meta_var, MatchResult, MetaVarEnv};
use crate::ts_parser::Edit;
use crate::{Node, Root};

/// Replace meta variable in the replacer string
pub trait Replacer {
    fn generate_replacement(&self, env: &MetaVarEnv) -> String;
}

impl<S: AsRef<str>> Replacer for S {
    fn generate_replacement(&self, env: &MetaVarEnv) -> String {
        let root = Root::new(self.as_ref());
        let mut stack = vec![root.root()];
        let mut edits = vec![];
        let mut reverse = vec![];
        // TODO: benchmark dfs performance
        while let Some(node) = stack.pop() {
            if let Some(text) = get_meta_var_replacement(&node, env) {
                let position = node.inner.start_byte();
                let length = node.inner.end_byte() - position;
                edits.push(Edit {
                    position,
                    deleted_length: length,
                    inserted_text: text,
                });
            } else {
                reverse.extend(node.children());
                stack.extend(reverse.drain(..).rev());
            }
        }
        // add the missing one
        edits.push(Edit {
            position: root.source.len(),
            deleted_length: 0,
            inserted_text: String::new(),
        });
        let mut ret = String::new();
        let mut start = 0;
        for edit in edits {
            ret.push_str(&root.source[start..edit.position]);
            ret.extend(edit.inserted_text.chars());
            start = edit.position + edit.deleted_length;
        }
        ret
    }
}

fn get_meta_var_replacement(node: &Node, env: &MetaVarEnv) -> Option<String> {
    if !node.is_leaf() {
        return None;
    }
    let meta_var = extract_meta_var(node.text())?;
    let replaced = match env.get(&meta_var)? {
        MatchResult::Single(replaced) => replaced.text().to_string(),
        MatchResult::Multi(nodes) => nodes.iter().flat_map(|n| n.text().chars()).collect(),
    };
    Some(replaced)
}

impl<'a> Replacer for Node<'a> {
    fn generate_replacement(&self, _: &MetaVarEnv) -> String {
        self.text().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Root;
    use std::collections::HashMap;

    fn test_str_replace(replacer: &str, vars: &[(&str, &str)], expected: &str) {
        let mut env = MetaVarEnv::new();
        let roots: Vec<_> = vars.iter().map(|(v, p)| (v, Root::new(p))).collect();
        for (var, root) in &roots {
            env.insert(var.to_string(), root.root());
        }
        let replaced = replacer.generate_replacement(&env);
        assert_eq!(replaced, expected, "wrong replacement {replaced} {expected} {:?}", HashMap::from(env));
    }

    #[test]
    fn test_no_env() {
        test_str_replace("let a = 123", &[], "let a = 123");
        test_str_replace("console.log('hello world'); let b = 123;", &[], "console.log('hello world'); let b = 123;");
    }

    #[test]
    fn test_single_env() {
        test_str_replace("let a = $A", &[("A", "123")], "let a = 123");
        test_str_replace("console.log($HW); let b = 123;", &[("HW", "'hello world'")], "console.log('hello world'); let b = 123;");
    }

    #[test]
    fn test_multiple_env() {
        test_str_replace("let $V = $A", &[("A", "123"), ("V", "a")], "let a = 123");
        test_str_replace(
            "console.log($HW); let $B = 123;",
            &[("HW", "'hello world'"), ("B", "b")],
            "console.log('hello world'); let b = 123;");
    }

    #[test]
    fn test_multiple_occurrences() {
        test_str_replace("let $A = $A", &[("A", "a")], "let a = a");
        test_str_replace("var $A = () => $A", &[("A", "a")], "var a = () => a");
        test_str_replace(
            "const $A = () => { console.log($B); $A(); };",
            &[("B", "'hello world'"), ("A", "a")],
            "const a = () => { console.log('hello world'); a(); };");
    }

    fn test_ellipsis_replace(replacer: &str, vars: &[(&str, &str)], expected: &str) {
        let mut env = MetaVarEnv::new();
        let roots: Vec<_> = vars.iter().map(|(v, p)| (v, Root::new(p))).collect();
        for (var, root) in &roots {
            env.insert_multi(var.to_string(), root.root().children().collect());
        }
        let replaced = replacer.generate_replacement(&env);
        assert_eq!(replaced, expected, "wrong replacement {replaced} {expected} {:?}", HashMap::from(env));
    }

    #[test]
    fn test_ellipsis_meta_var() {
        test_ellipsis_replace("let a = () => { $$$B }", &[("B", "alert('works!')")], "let a = () => { alert('works!') }");
        test_ellipsis_replace("let a = () => { $$$B }", &[("B", "alert('works!');console.log(123)")], "let a = () => { alert('works!');console.log(123) }");
    }
}