use std::{
    io::Read,
    fs::File,
};

use regex::Regex;

pub fn make_document(text: &str) -> Vec<String> {
    let re = Regex::new(r"\n\s*\n").unwrap();
    let paragraphs: Vec<String> = re.split(text).map(|s| s.trim().to_string()).collect();
    paragraphs
}


pub fn rank_documents(docs: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let  mut ndocs = docs.clone();
    ndocs.sort_by(|a,b| b.len().cmp(&a.len()));
    ndocs
}

// fn main() {
//     let fox = "The quick brown fox jumps over the lazy dog.";
//     let para3 = "a\n\nb\n\nc";
//     let bustle = "\
//         The bustle in a house\n\
//         The morning after death\n\
//         Is solemnest of industries\n\
//         Enacted upon earth,—\n\
//         \n\
//         The sweeping up the heart,\n\
//         And putting love away\n\
//         We shall not want to use again\n\
//         Until eternity.\n\
//         ";
//     let doc1 = make_document(fox); // 1 paragraph
//     let doc2 = make_document(bustle); // 2 paragraphs
//     let doc3 = make_document(para3);

//     let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
//     let rnk_docs = rank_documents(&docs);
//     assert_eq!(rnk_docs, [doc3, doc2, doc1]);
// }

pub fn read_to_doc(filename: &[String]) -> Vec<(String, Vec<String>)> {
    let mut list_docs = Vec::new();
    // let file = File::open(filename);
    for file_path in filename {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error opening file: {}", file_path);
                return list_docs;
            }
        };
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let doc = make_document(&content);
        list_docs.push((file_path.to_string(), doc))
    }
    list_docs.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    list_docs
}

pub fn generate_html(v: &Vec<(String, Vec<String>)>) -> String {
    let mut html = String::from("<style>\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>");
    html.push_str("\n<table>");
    html.push_str("\n\t<tr>\n\t<th>File</th>\n\t<th>Number of paragraps</th>\n\t</tr>");
    
    for (name, docs) in v {
        html.push_str("\n\t<tr>");
        html.push_str(&format!("\n\t\t<td>{}</td>\n\t\t<td>{}</td>", name, docs.len()));
        html.push_str("\n\t</tr>");
    }
    html.push_str("\n</table>");
    html
}