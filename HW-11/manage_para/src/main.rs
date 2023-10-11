use std::io::Write;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Please input at least one file name.");
        return;
    }

    let files: Vec<String> = args.into_iter().skip(1).collect();
    let docs = manage_para::read_to_doc(&files);
    let html = manage_para::generate_html(&docs);

    let mut file_html = File::create("output.html").unwrap();
    let _ = file_html.write_all(html.as_bytes());

    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,—\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
        ";
    let doc1 = manage_para::make_document(fox); // 1 paragraph
    let doc2 = manage_para::make_document(bustle); // 2 paragraphs
    let doc3 = manage_para::make_document(para3);

    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = manage_para::rank_documents(&docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}

