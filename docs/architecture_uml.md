# zpr-scan-search — Architecture UML

---

## 1 · Python Bindings & Orchestration

```mermaid
classDiagram
    direction TB

    class PyClient {
        -inner: Client
        +new(paths: PyList) PyResult~Self~
        +search(query: PyQuery) PyResult
        +sem_search(query: PyQuery) PyResult
    }

    class PyQuery {
        -inner: Query
        +new(term, before, after) Self
    }

    class PySearchResult {
        -inner: SearchResult
        +before() str
        +matched() str
        +after() str
    }

    class Client {
        -files: Vec~Arc~TextFile~~
        -config: ScanSearchConfig
        +new(config, paths) Result~Self~
        +search(query: Query) Result
        +sem_search(query: Query) Result
    }

    PyClient *-- Client : wraps
    PyQuery *-- Query : wraps
    PySearchResult *-- SearchResult : wraps
```

---

## 2 · Configuration

```mermaid
classDiagram
    direction TB

    class ScanSearchConfig {
        +fs_scan: FsScanConfig
        +cache_config: CacheConfig
        +search_config: SearchConfig
        +ocr_config: OcrConfig
    }

    class FsScanConfig {
        +follow_links: bool
        +include_hidden: bool
    }

    class CacheConfig {
        <<enum>>
        Local
        Global(path: PathBuf)
        +build() Box~dyn CacheBackend~
    }

    class SearchConfig {
        +sem_search: bool
    }

    class OcrConfig {
        +languages: Vec~String~
    }

    ScanSearchConfig *-- FsScanConfig
    ScanSearchConfig *-- CacheConfig
    ScanSearchConfig *-- SearchConfig
    ScanSearchConfig *-- OcrConfig
```

---

## 3 · Search Core & Implementations

```mermaid
classDiagram
    direction TB

    class Search {
        <<trait>>
        +search(query: Query)*
        _Returns Result~impl Iterator~SearchResult~~_
    }

    class Query {
        +term: String
        +context: SearchContext
    }

    class SearchContext {
        +before: usize
        +after: usize
    }

    class SearchResult {
        +before: ArcStrSlice
        +matched: ArcStrSlice
        +after: ArcStrSlice
        +before() str
        +matched() str
        +after() str
    }

    class ArcStrSlice {
        -text: Arc~str~
        -range: Range~usize~
        +new(text, range) Self
        +as_str() str
    }

    Query *-- SearchContext
    SearchResult *-- ArcStrSlice

    class IndexSearcher {
        -file: Arc~TextFile~
        +new(file) Self
    }

    class IndexSearcherIterator {
        -text: Arc~str~
        -locations: Vec~i32~
        -query_length: usize
        -pos: usize
        -word_pos: usize
        -byte_pos: usize
        -context: SearchContext
    }

    IndexSearcher ..|> Search : implements
    IndexSearcher ..> IndexSearcherIterator : creates
    IndexSearcher o-- TextFile : Arc

    class SemSearcher~E~ {
        -file: Arc~TextFile~
        -encoder: E
        -queue_size: usize
        +new(file, encoder, queue_size) Self
    }

    class SemSearcherIterator {
        -text: Arc~str~
        -locations: Vec~i32~
        -pos: usize
        -line_pos: usize
        -byte_pos: usize
        -context: SearchContext
    }

    class CosinedEmbedding {
        -similarity: OrderedFloat~f32~
        -location: i32
    }

    SemSearcher ..|> Search : implements
    SemSearcher ..> SemSearcherIterator : creates
    SemSearcher ..> CosinedEmbedding : uses internally
    SemSearcher o-- TextFile : Arc
    SemSearcher --> TextEncoder : encodes query via
```

---

## 4 · File & Loader

```mermaid
classDiagram
    direction TB

    class TextFile {
        -path: PathBuf
        -text: Arc~str~
        -map: Arc~WordMap~
        -embeddings: Arc~Option~Embeddings~~
        +new(path, text, map, embeddings) Self
        +path() Path
        +get(key: str) Option~Vec~i32~~
        +text() str
        +text_arc() Arc~str~
        +map() WordMap
        +embeddings() Arc~Option~Embeddings~~
    }

    class FileLoader {
        <<trait>>
        +load(file: SupportedFile, embed: bool) Result~TextFile~
    }

    class TextFileLoader~E_B_C~ {
        -extractor: E  «TextExtractor»
        -backend: B   «CacheBackend»
        -encoder: C   «TextEncoder»
        +new(extractor, backend, encoder) Self
    }

    TextFileLoader ..|> FileLoader : implements
    TextFileLoader ..> TextFile : produces
    TextFileLoader ..> TextExtractor : extracts text via
    TextFileLoader ..> CacheBackend : reads/writes cache via
    TextFileLoader ..> TextEncoder : generates embeddings via
```

---

## 5 · Extractor, Encoder & OCR

```mermaid
classDiagram
    direction TB

    class TextEncoder {
        <<trait>>
        +encode(text: slice~str~) Result~Embeddings~
    }

    class FastEmbed {
        +encode(text) Result~Embeddings~
    }

    FastEmbed ..|> TextEncoder : implements

    class TextExtractor {
        <<trait>>
        +extract_from(file: SupportedFile) Result~String~
    }

    class UniversalExtractor~E~ {
        -engine: Arc~E «OcrEngine»~
        +new(engine) Self
    }

    class PdfExtractor~E~ {
        -ocr_engine: Arc~E «OcrEngine»~
        +new(ocr_engine) Self
    }

    class ImageTextExtractor~E~ {
        -engine: Arc~E «OcrEngine»~
        +new(engine) Self
    }

    UniversalExtractor ..|> TextExtractor : implements
    PdfExtractor ..|> TextExtractor : implements
    ImageTextExtractor ..|> TextExtractor : implements

    UniversalExtractor ..> PdfExtractor : delegates PDF
    UniversalExtractor ..> ImageTextExtractor : delegates Image

    PdfExtractor --> OcrEngine : uses
    ImageTextExtractor --> OcrEngine : uses

    class OcrEngine {
        <<trait>>
        +extract_text_from_image(image: DynamicImage) Result~String~
    }

    class TesseractEngine {
        -tess_pool: ThreadLocal~TesseractAPI~
        -tessdata_path: String
        -lang: String
        +new(config: OcrConfig) Result~Self~
    }

    TesseractEngine ..|> OcrEngine : implements
```

---

## 6 · Caching Subsystem

```mermaid
classDiagram
    direction TB

    class CacheBackend {
        <<trait>>
        +try_load(path, fingerprint, reload_cache) Result~Option~CachedDocument~~
        +submit_job(path, job)
    }

    class LocalCache {
        +try_load(path, fingerprint, reload_cache) Result~Option~CachedDocument~~
        +submit_job(path, job)
    }

    class GlobalCache {
        +path: PathBuf
        +try_load(path, fingerprint, reload_cache) Result~Option~CachedDocument~~
        +submit_job(path, job)
    }

    LocalCache ..|> CacheBackend : implements
    GlobalCache ..|> CacheBackend : implements

    class CacheWriter {
        -tx: Sender~Msg~
        +get() CacheWriter  «singleton»
        +submit(msg: Msg)
        +shutdown()
    }

    class Msg {
        <<enum>>
        Write‑WriteTask
        Shutdown‑Sender
    }

    class WriteTask {
        +path: PathBuf
        +data: Vec~u8~
    }

    LocalCache ..> CacheWriter : submits via

    CacheWriter ..> WriteTask : processes

    Msg *-- WriteTask

    class Job {
        <<enum>>
        CacheWrite
    }

    class CachedDocument {
        +text: String
        +map: WordMap
        +fingerprint: FileFingerprint
        +embeddings: Option~Embeddings~
    }

    class FileFingerprint {
        +mtime_secs: u64
        +mtime_nanos: u32
        +size: u64
        +from_path(path) Result~Self~
    }

    class WordMap {
        -HashMap~String‑Vec~i32~~
        +new() Self
        +from(text: str) Self
    }

    class Embeddings {
        -Vec~Vec~f32~~
        +new() Self
    }

    CachedDocument *-- WordMap
    CachedDocument *-- FileFingerprint
    CachedDocument *-- Embeddings

```

---

## 7 · File Detection

```mermaid
classDiagram
    direction TB

    class MimeDetector {
        <<trait>>
        +detect(path: Path) Option~str~
    }

    class InferDetector {
        +detect(path) Option~str~
    }

    InferDetector ..|> MimeDetector : implements

    class SupportedFile {
        +path: PathBuf
        +kind: FileKind
        +from_path(path, detector) Option~Self~
    }

    class FileKind {
        <<enum>>
        Pdf
        Image
    }

    SupportedFile *-- FileKind
    SupportedFile ..> MimeDetector : uses
```


---

## Search Flow

```mermaid
flowchart TB
    subgraph init ["Initialization (Client::new)"]
        direction TB
        A["Receive file paths"] --> B["Scan directories (dir_utils)"]
        B --> C["Detect supported files (InferDetector)"]
        C --> D{"For each SupportedFile"}
        D --> E["Check cache (LocalCache / GlobalCache)"]
        E -->|Cache hit| F["Load TextFile from cache"]
        E -->|Cache miss| G["Extract text (UniversalExtractor)"]
        G --> G1{"FileKind?"}
        G1 -->|PDF| G2["PdfExtractor + OCR"]
        G1 -->|Image| G3["ImageTextExtractor + OCR"]
        G2 --> H["Build WordMap"]
        G3 --> H
        H --> I["Generate embeddings (FastEmbed)"]
        I --> J["Write to cache (CacheWriter)"]
        J --> F
        F --> K["Store Arc TextFile"]
    end

    subgraph search ["Exact Search (Client::search)"]
        direction TB
        L["Receive Query"] --> M["For each TextFile (parallel)"]
        M --> N["IndexSearcher: lookup term in WordMap"]
        N --> O["Find rarest word, verify phrase positions"]
        O --> P["Build SearchResult with context"]
    end

    subgraph sem ["Semantic Search (Client::sem_search)"]
        direction TB
        Q["Receive Query"] --> R["For each TextFile (parallel)"]
        R --> S["SemSearcher: encode query (FastEmbed)"]
        S --> T["Cosine similarity vs line embeddings"]
        T --> U["BinaryHeap → top-N matches"]
        U --> V["Build SearchResult with context"]
    end

    init --> search
    init --> sem
```
