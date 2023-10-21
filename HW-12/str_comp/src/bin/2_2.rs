fn main() {}

trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}

impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct PlainText {
    chars: String,
}

impl From<&str> for PlainText {
    fn from(value: &str) -> Self {
        PlainText {
            chars: value.to_string(),
        }
    }
}

impl Text for PlainText {
    fn value(&self) -> String {
        self.chars.clone()
    }
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

#[derive(Clone)]
struct RepeatedText {
    text: Box<dyn Text>,
    n: usize,
}
impl Text for RepeatedText {
    fn value(&self) -> String {
        self.text.value().repeat(self.n)
    }
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl Text for Box<dyn Text> {
    fn value(&self) -> String {
        self.as_ref().value()
    }
    fn clone_box(&self) -> Box<dyn Text> {
        self.as_ref().clone_box()
    }
}

impl RepeatedText {
    fn with_parts(text: &dyn Text, n: usize) -> RepeatedText {
        RepeatedText {
            text: text.clone_box(),
            n,
        }
    }
}

// -----------------------------------------------------------------------------------------

#[derive(Clone)]
struct JoinedText {
    text: Vec<Box<dyn Text>>,
    join: PlainText,
}

impl Text for JoinedText {
    fn value(&self) -> String {
        let mut result = Vec::new();
        for text in self.text.clone() {
            result.push(text.value());
        }
        result.join(&self.join.value())
    }
    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl JoinedText {
    fn with_parts(text: &Vec<Box<dyn Text>>, join: &PlainText) -> JoinedText {
        JoinedText { text: text.to_vec(), join: join.clone() }
    }
}

#[test]
fn test_text_composition() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}
