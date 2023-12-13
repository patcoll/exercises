#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Type {
    Insert,
    Delete,
    Skip,
    #[default]
    Noop,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Operation {
    op: Type,
    count: isize,
    chars: String,
}

impl Operation {
    pub fn insert(chars: &str) -> Self {
        Self::from_op_and_count_and_chars("insert", Default::default(), chars)
    }

    pub fn skip(count: isize) -> Self {
        Self::from_op_and_count_and_chars("skip", count, Default::default())
    }

    pub fn delete(count: isize) -> Self {
        Self::from_op_and_count_and_chars("delete", count, Default::default())
    }

    pub fn noop() -> Self {
        Self::from_op_and_count_and_chars("noop", Default::default(), Default::default())
    }

    pub fn from_op_and_count_and_chars(op: &str, count: isize, chars: &str) -> Self {
        Operation {
            op: Self::get_op_type(op),
            count,
            chars: chars.to_owned(),
        }
    }

    fn get_op_type(op: &str) -> Type {
        match op {
            "insert" => Type::Insert,
            "delete" => Type::Delete,
            "skip" => Type::Skip,
            _ => Type::Noop,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
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
}
