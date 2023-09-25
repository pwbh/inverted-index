# Inverted index

This is a concurrent implementation of the Inverted Index data structure using Rust programming language.

<p align="center">
  <img src="https://codefying.files.wordpress.com/2015/12/simpleinvertedindex.jpg?w=640" style="border-radius: 5px;">
</p>

### A quote about inverted index from Wikipedia

"In computer science, an inverted index (also referred to as a postings list, postings file, or inverted file) is a database index storing a mapping from content, such as words or numbers, to its locations in a table, or in a document or a set of documents (named in contrast to a forward index, which maps from documents to content).

The purpose of an inverted index is to allow fast full-text searches, at a cost of increased processing when a document is added to the database. The inverted file may be the database file itself, rather than its index.

It is the most popular data structure used in document retrieval systems, used on a large scale for example in search engines. Additionally, several significant general-purpose mainframe-based database management systems have used inverted list architectures, including ADABAS, DATACOM/DB, and Model 204."

Currently this implementation uses a plain resizable array for storing the indexes, I may change in the future for something a little more appropriate. Feel free to send in a PR.

## Usage

```rust
fn main() {
    let file_path = "./my_files/texts/some_text.txt"
    let inverted_index = inverted_index::InvertedIndex::new(file_path);
    inverted_index.index(100).unwrap();
    println!("{}", inverted_index);
}
```

## Performance

| LoT    | Single Thread | Multi Thread (100 worker threads) |
| ------ | ------------- | -------------------------- |
| 80,000 | 3m 49.75s     | 3.686s                     |
