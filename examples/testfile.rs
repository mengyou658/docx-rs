use docx::DocxFile;
use docx::document::Paragraph;

fn main() {
    let docx = DocxFile::from_file(String::from("hello_world.docx")).unwrap();
    let mut docx = docx.parse().unwrap();

    let para = Paragraph::default().push_text("Lorem Ipsum");
    docx.document.push(para);
    for doc in &docx.document.body.content {
        println!("{:?}", doc);
    }
}
