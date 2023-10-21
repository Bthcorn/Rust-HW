#[derive(Clone, Debug)]
enum Text {
    Plain(String),
    Repeated(Box<Text>, usize),
    Joined(Vec<Box<Text>>, Box<Text>),
}

impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(t, n) => t.value().clone().repeat(*n),
            Text::Joined(t1, t2) => {
                let mut result = Vec::new();
                for text in t1.clone() {
                    result.push(text.value());
                }
                result.join(&t2.value().clone())
            }
        }
    }
}

impl From<&Text> for Box<Text> {
    fn from(value: &Text) -> Self {
        Box::new(value.clone())
    }
}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text {
        &self
    }
}

#[test]
fn test_text_composition() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}

 fn main() {}