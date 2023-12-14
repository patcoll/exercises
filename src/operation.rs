use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Insert,
    Delete,
    Skip,
    #[default]
    Noop,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Operation {
    op: Type,
    count: isize,
    chars: String,
}

impl Operation {
    pub fn insert(chars: &str) -> Self {
        Self { op: Type::Insert, chars: chars.to_owned(), ..Default::default() }
    }

    pub fn skip(count: isize) -> Self {
        Self { op: Type::Skip, count, ..Default::default() }
    }

    pub fn delete(count: isize) -> Self {
        Self { op: Type::Delete, count, ..Default::default() }
    }

    pub fn noop() -> Self {
        Self { op: Type::Noop, ..Default::default() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Document {
    pub pos: usize,
    pub content: String,
}

impl Document {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn transform(&mut self, operation: &Operation) -> &mut Self {
        match operation.op {
            Type::Insert => {
                let before = &self.content[0..self.pos];
                let after = &self.content[self.pos..self.content.len()];
                let updated_content = before.to_owned() + &operation.chars + after;

                self.content = updated_content;
                self.pos += operation.chars.len();
                self
            },
            Type::Delete => {
                let tmp_pos: isize = self.pos as isize + operation.count;

                let content_len_isize: isize = self.content.len() as isize;

                if tmp_pos >= 0 && tmp_pos <= content_len_isize {
                    let before = &self.content[0..self.pos];
                    let after = &self.content[(tmp_pos as usize)..self.content.len()];
                    let updated_content = before.to_owned() + after;
                    self.content = updated_content;
                }

                self
            },
            Type::Skip => self.skip(operation.count),
            Type::Noop => self,
        }
    }

    pub fn skip(&mut self, count: isize) -> &mut Self {
        let tmp_pos: isize = self.pos as isize + count;

        self.pos =
            if tmp_pos > self.content.len().try_into().unwrap() {
                self.content.len()
            } else if tmp_pos < 0 {
                0
            } else {
                tmp_pos.try_into().unwrap()
            };

        self
    }
}

impl From<&str> for Document {
    fn from(content: &str) -> Self {
        Self { content: content.to_string(), ..Default::default() }
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Eq for Document {}

#[derive(Debug, Default)]
struct Verify {
    stale: Document,
    latest: Document,
    operations: Vec<Operation>,
}

#[allow(dead_code)] // somehow needed because only used in tests
impl Verify {
    pub fn new(stale: &str, latest: &str, operations_json: &str) -> Self {
        Verify {
            stale: Document::from(stale),
            latest: Document::from(latest),
            operations: serde_json::from_str(operations_json).unwrap(),
        }
    }

    pub fn execute(&self) -> bool {
        let mut stale_doc = self.stale.clone();

        for operation in &self.operations {
            stale_doc.transform(operation);
        }

        stale_doc == self.latest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_document() {
        let mut document = Document::new();

        assert_eq!(document.pos, 0);
        assert_eq!(document.content, "".to_string());

        document.transform(&Operation::noop());

        assert_eq!(document.pos, 0);
        assert_eq!(document.content, "".to_string());

        document.transform(&Operation::insert("We"));

        assert_eq!(document.pos, 2);
        assert_eq!(document.content, "We".to_string());

        document.transform(&Operation::insert(" need"));

        assert_eq!(document.pos, 7);
        assert_eq!(document.content, "We need".to_string());

        document.transform(&Operation::insert(" to talk"));

        assert_eq!(document.pos, 15);
        assert_eq!(document.content, "We need to talk".to_string());

        document.transform(&Operation::skip(-4));

        assert_eq!(document.pos, 11);
        assert_eq!(document.content, "We need to talk".to_string());

        document.transform(&Operation::insert("really "));

        assert_eq!(document.pos, 18);
        assert_eq!(document.content, "We need to really talk".to_string());

        document.transform(&Operation::delete(4));

        assert_eq!(document.pos, 18);
        assert_eq!(document.content, "We need to really ".to_string());

        document.transform(&Operation::skip(-1));
        document.transform(&Operation::delete(1));

        assert_eq!(document.pos, 17);
        assert_eq!(document.content, "We need to really".to_string());

        document.transform(&Operation::delete(1));

        assert_eq!(document.pos, 17);
        assert_eq!(document.content, "We need to really".to_string());
    }

    #[test]
    fn test_document_cannot_skip_past_beginning() {
        let mut document = Document::new();

        assert_eq!(document.pos, 0);
        assert_eq!(document.content, "".to_string());

        document.transform(&Operation::skip(-40));

        assert_eq!(document.pos, 0);
        assert_eq!(document.content, "".to_string());
    }

    #[test]
    fn test_document_cannot_skip_beyond_end() {
        let mut document = Document::new();

        assert_eq!(document.pos, 0);
        assert_eq!(document.content, "".to_string());

        document.transform(&Operation::skip(40));

        assert_eq!(document.pos, 0);
        assert_eq!(document.content, "".to_string());
    }

    #[test]
    fn test_operation() {
        assert_eq!(
            Operation::insert("We"),
            Operation{op: Type::Insert, count: 0, chars: "We".to_string()}
        );
        assert_eq!(
            Operation::skip(40),
            Operation{op: Type::Skip, count: 40, chars: Default::default()}
        );
        assert_eq!(
            Operation::delete(40),
            Operation{op: Type::Delete, count: 40, chars: Default::default()}
        );
    }

    #[test]
    fn test_operations_deserialize() {
        let data = r#"[{"op": "skip", "count": 40}, {"op": "delete", "count": 47}, {"op": "insert", "chars": "hello"}]"#;
        let ops: Vec<Operation> = serde_json::from_str(data).unwrap();
        assert_eq!(ops, vec![Operation::skip(40), Operation::delete(47), Operation::insert("hello")]);
    }

    #[test]
    fn test_verify_works() {
        let verify = Verify::new(
            "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
            "Repl.it uses operational transformations.",
            r#"[{"op": "skip", "count": 40}, {"op": "delete", "count": 47}]"#
        );

        assert!(verify.execute());
    }
}
