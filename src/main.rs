fn main() -> Result<(), String> {
    let collection = vec![
        "./documents/1.doc.txt",
        "./documents/2.doc.txt",
        "./documents/post.doc.txt",
        "./documents/shakespeare.doc.txt",
    ];

    let inverted_index = inverted_index::InvertedIndex::new();

    collection.iter().for_each(|p| {
        inverted_index.index(p, 100).unwrap();
        println!("{}", inverted_index);
    });

    Ok(())
}
