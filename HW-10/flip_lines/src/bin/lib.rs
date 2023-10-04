fn vflip(text: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut iter = text.iter();
    while let Some(last) = iter.next_back() {
        result.push(last.to_string())
    }
    result
}

fn hflip(text: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut iter = text.iter();
    let mut max = 0;

    for j in text {
        if j.len() >= max {
            max = j.len();
        }
    }

    while let Some(text) = iter.next() {
        let mut ntext = String::new();
        if text.len() < max {
            for i in 0..max-text.len() {
                ntext.push_str(" ");
            }
        }
        for t in text.chars().rev() {
            ntext.push(t);
        }
        result.push(ntext.to_string())
    }
    result
}

fn main() {
    // Create a String with an initial capacity of 10 characters
    let mut my_string = String::with_capacity(10);

    // You can append characters to the String as needed
    my_string.push('a');
    my_string.push('b');
    my_string.push('c');

    println!("String:{}", my_string);
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vflip(&data), ["<==", "#####", "<--"]);
    assert_eq!(hflip(&data), ["  --<", "#####", "  ==<"]);
}