use tantivy::schema::*;
use tantivy::{Index, IndexWriter, TantivyDocument, IndexReader};
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use std::sync::{Mutex, RwLock, Arc};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::time::{Duration, Instant};

pub struct MySearchEngine {
    indexes: RwLock<HashMap<String, Arc<SingleIndex>>>,
}

pub struct SingleIndex {
    index: Index,
    writer: Mutex<IndexWriter>,
    reader: RwLock<IndexReader>,
}

impl SingleIndex {
    fn new() -> tantivy::Result<Self> {
        let mut schema_builder = Schema::builder();
        let _title = schema_builder.add_text_field("title", TEXT | STORED);
        let _body = schema_builder.add_text_field("body", TEXT | STORED);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());
        let writer = index.writer(1024 * 1024 * 1024)?;
        let reader = index.reader()?;

        Ok(SingleIndex {
            index,
            writer: Mutex::new(writer),
            reader: RwLock::new(reader),
        })
    }

    fn refresh_reader(&self) -> tantivy::Result<()> {
        let new_reader = self.index.reader()?;
        *self.reader.write().unwrap() = new_reader;
        Ok(())
    }

    fn add_document(&self, title: &str, body: &str) -> tantivy::Result<Duration> {
        let start = Instant::now();
        let mut writer = self.writer.lock().unwrap();
        let mut doc = TantivyDocument::default();
        doc.add_text(self.index.schema().get_field("title").unwrap(), title);
        doc.add_text(self.index.schema().get_field("body").unwrap(), body);
        writer.add_document(doc).expect("Failed to add document");
        writer.commit()?;
        self.refresh_reader()?;
        let duration = start.elapsed();
        Ok(duration)
    }

    fn add_documents(&self, documents: &Vec<(String, String)>) -> tantivy::Result<Duration> {
        let start = Instant::now();
        let mut writer = self.writer.lock().unwrap();
        for (title, body) in documents {
            let mut doc = TantivyDocument::default();
            doc.add_text(self.index.schema().get_field("title").unwrap(), title);
            doc.add_text(self.index.schema().get_field("body").unwrap(), body);
            writer.add_document(doc).expect("Failed to add document");
        }
        writer.commit()?;
        self.refresh_reader()?;
        let duration = start.elapsed();
        Ok(duration)
    }

    fn search(&self, query_str: &str) -> tantivy::Result<(Duration, Duration, Vec<(f32, TantivyDocument)>)> {
        let start1 = Instant::now();
        let reader = self.reader.read().unwrap();
        let duration2 = start1.elapsed();
        let searcher = reader.searcher();

        let query_parser = QueryParser::for_index(&self.index, vec![
            self.index.schema().get_field("body").unwrap(),
        ]);
        let query = query_parser.parse_query(query_str)?;

        let start = Instant::now();
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
        let duration = start.elapsed();
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            results.push((score, doc));
        }

        Ok((duration, duration2, results))
    }
}

impl MySearchEngine {
    fn new() -> Self {
        MySearchEngine {
            indexes: RwLock::new(HashMap::new()),
        }
    }

    fn get_or_create_index(&self, key: &str) -> tantivy::Result<Arc<SingleIndex>> {
        let mut indexes = self.indexes.write().unwrap();

        if let Some(index) = indexes.get(key) {
            return Ok(Arc::clone(index));
        }

        let index = SingleIndex::new()?;
        let index = Arc::new(index);
        indexes.insert(key.to_string(), Arc::clone(&index));
        Ok(index)
    }

    pub fn add_document(&self, key: &str, title: &str, body: &str) -> tantivy::Result<Duration> {
        let index = self.get_or_create_index(key)?;
        index.add_document(title, body)
    }

    pub fn add_documents(&self, key: &str, documents: &Vec<(String, String)>) -> tantivy::Result<Duration> {
        let index = self.get_or_create_index(key)?;
        index.add_documents(documents)
    }

    pub fn search(&self, key: &str, query_str: &str) -> tantivy::Result<(Duration, Duration, Vec<(f32, TantivyDocument)>)> {
        let index = self.get_or_create_index(key)?;
        index.search(query_str)
    }
}

lazy_static! {
    pub static ref SEARCH_ENGINE: Arc<MySearchEngine> = {
        let engine = MySearchEngine::new();
        Arc::new(engine)
    };
}

fn main() -> tantivy::Result<()> {
    let engine = SEARCH_ENGINE.clone();

    // Add a single document
    engine.add_document("index1", "First Title", "First Body")?;

    // Add multiple documents
    let documents = vec![
        ("Second Title".to_string(), "Second Body".to_string()),
        ("Third Title".to_string(), "Third Body".to_string()),
    ];
    engine.add_documents("index1", &documents)?;

    // Search
    let (search_duration, reader_creation_duration, results) = engine.search("index1", "Body")?;
    println!("Search took: {:?}", search_duration);
    println!("Reader creation took: {:?}", reader_creation_duration);
    for (score, doc) in results {
        println!("Score: {} Doc: {:?}", score, doc);
    }

    Ok(())
}
