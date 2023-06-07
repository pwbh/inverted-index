use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
    sync::{Arc, Mutex},
    thread,
};

type Postings = HashSet<String>;

#[derive(Debug)]
pub struct InvertedIndex {
    indices: Arc<Mutex<HashMap<String, Postings>>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            indices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn index(&self, document_path: &str, thread_count: usize) -> Result<(), String> {
        let content = match fs::read_to_string(document_path.clone()) {
            Ok(s) => Arc::new(s),
            Err(e) => return Err(e.to_string()),
        };

        let mut thread_handles = vec![];

        let lines_count = content.lines().count();

        let max_threads_needed = if lines_count < thread_count && lines_count != 0 {
            lines_count
        } else {
            thread_count
        };

        let leftover = lines_count % max_threads_needed;
        let lines_per_thread = (lines_count - leftover) / max_threads_needed;

        for i in 0..max_threads_needed {
            let content = Arc::clone(&content);
            let indices = Arc::clone(&self.indices);
            let document_path = document_path.to_string();

            let thread_handle = thread::spawn(move || {
                let lines: Vec<_> = content.lines().collect();

                let start = i * lines_per_thread;
                let end = if i == max_threads_needed - 1 {
                    i * lines_per_thread + lines_per_thread + leftover
                } else {
                    i * lines_per_thread + lines_per_thread
                };

                let results = index_document_for_thread(&document_path, &lines[start..end]);

                let mut indices = indices.lock().unwrap();

                for (key, value) in results.iter() {
                    indices
                        .entry(key.to_string())
                        .and_modify(|e| e.extend(value.clone().into_iter()))
                        .or_insert(value.clone());
                }
            });

            thread_handles.push(thread_handle)
        }

        for (i, thread) in thread_handles.into_iter().enumerate() {
            match thread.join() {
                Ok(_) => {}

                Err(_) => {
                    return Err(format!("Something wen't wrong in thread {}", i + 1));
                }
            }
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        let indices = self.indices.lock().unwrap();
        return indices.len();
    }
}

impl Display for InvertedIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} inverted indexes in memory", self.len())
    }
}

fn index_document_for_thread(document_path: &str, lines: &[&str]) -> Vec<(String, Postings)> {
    let mut indices: Vec<(String, Postings)> = vec![];

    for line in lines {
        for word in line.split_ascii_whitespace() {
            let key = if word.len() == 1 {
                word.chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .next()
                    .unwrap()
                    .to_string()
            } else {
                word.to_lowercase()
            };

            if let Some(result) = indices.iter_mut().find(|f| f.0 == key) {
                if let None = result.1.iter().find(|p| **p == document_path) {
                    result.1.insert(document_path.to_string());
                }
            } else {
                indices.push((key.to_string(), HashSet::from([document_path.to_string()])));
            }
        }
    }

    // indices.sort_by(|a, b| a.0.cmp(&b.0));

    return indices;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_file(path: &str, content: &str) -> String {
        fs::write(path, content).unwrap();
        return path.to_string();
    }

    fn delete_test_file(path: &str) {
        fs::remove_file(path).unwrap()
    }

    #[test]
    fn inverted_index_indexes_document() {
        let file_path = "./inverted_index_indexes_document_1.txt";

        create_test_file(file_path, "Hello its an inverted index test file. Just to see how it works and indexes this file.");

        let inverted_index = InvertedIndex::new();
        let result = inverted_index.index(file_path, 1);

        assert!(result.is_ok());
        assert_eq!(inverted_index.len(), 16);
        delete_test_file(file_path);
    }

    #[test]
    fn inverted_index_doesnt_add_same_words() {
        let file_path = "./inverted_index_doesnt_add_same_words_1.txt";
        let file_path_2 = "./inverted_index_doesnt_add_same_words_2.txt";

        create_test_file(file_path, "Hello its an inverted index test file. Just to see how it works and indexes this file.");
        create_test_file(file_path_2, "Hello");

        let inverted_index = InvertedIndex::new();
        let result_1: Result<(), String> = inverted_index.index(file_path, 1);
        let result_2 = inverted_index.index(file_path_2, 1);

        assert!(result_1.is_ok());
        assert!(result_2.is_ok());
        assert_eq!(inverted_index.len(), 16);

        let indices = inverted_index.indices.lock().unwrap();
        let token = indices.get("hello");

        assert_eq!(
            token,
            Some(&HashSet::from([
                file_path.to_owned(),
                file_path_2.to_owned()
            ]))
        );

        delete_test_file(file_path);
        delete_test_file(file_path_2);
    }

    #[test]
    fn inverted_index_adds_unique() {
        let file_path_1 = "./inverted_index_adds_unique_1.txt";
        let file_path_2 = "./inverted_index_adds_unique_2.txt";
        let file_path_3 = "./inverted_index_adds_unique_3.txt";

        create_test_file(file_path_1, "Hello its an inverted index test file. Just to see how it works and indexes this file.");
        create_test_file(file_path_2, "boss.");
        create_test_file(file_path_3, "jOker");

        let inverted_index = InvertedIndex::new();
        let result_1 = inverted_index.index(file_path_1, 1);
        let result_2 = inverted_index.index(file_path_2, 1);
        let result_3 = inverted_index.index(file_path_3, 1);

        assert!(result_1.is_ok());
        assert!(result_2.is_ok());
        assert!(result_3.is_ok());
        assert_eq!(inverted_index.len(), 18);

        delete_test_file(file_path_1);
        delete_test_file(file_path_2);
        delete_test_file(file_path_3);
    }
}
