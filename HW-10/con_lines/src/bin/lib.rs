fn vcat(text1: &[String], text2: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut t1 = text1.iter();
    let mut t2 = text2.iter();
    while let Some(text) = t1.next() {
        result.push(text.to_string());
    }

    while let Some(text) = t2.next() {
        result.push(text.to_string());
    }
    result
}

fn hcat(text1: &[String], text2: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut max = 0;
    let size = vec![text1.len(), text2.len()];
    let mut max2 = 0;

    for j in text1 {
        if j.len() >= max2 {
            max2 = j.len();
        }
    }

    for j in size {
        if j >= max {
            max = j;
        }
    }

    if text1.is_empty() && text2.is_empty() {
        return result;
    } else {
        for i in 0..max {
            let mut ntext = "".to_string();

            let a = if i < text1.len() {
                text1[i].clone()
            } else {
                "".to_string()
            };
            ntext.push_str(&a);

            let b = if i < text2.len() {
                text2[i].clone()
            } else {
                "".to_string()
            };
            if b != "".to_string() {
                while ntext.len() < max2 {
                    ntext.insert_str(ntext.len(), " ");
                }
            }
            ntext.push_str(&b);
            result.push(ntext)
        }
    }
    result
}

fn main() {
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    let result = hcat(&data, &data[..2]);
    println!("{:?}", result);
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(
        vcat(&data, &data),
        ["<--", "#####", "<==", "<--", "#####", "<=="]
    );
    assert_eq!(hcat(&data, &data[..2]), ["<--  <--", "##########", "<=="]);
    assert_eq!(hcat(&data[..2], &data), ["<--  <--", "##########", "     <=="]);
}
