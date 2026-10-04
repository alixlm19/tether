CREATE TABLE cached_queries (
	id INTEGER PRIMARY KEY,
	query TEXT NOT NULL,
	model_name TEXT NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cached_responses (
	id INTEGER PRIMARY KEY,
	response_text TEXT,
	query_id INTEGER NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(query_id) REFERENCES cached_queries(id) ON DELETE CASCADE
);

CREATE TABLE document_dependencies (
	id INTEGER PRIMARY KEY,
	cached_response_id INTEGER NOT NULL,
	chunk_id TEXT,
	version_hash TEXT,
	FOREIGN KEY(cached_response_id) REFERENCES cached_responses(id) ON DELETE CASCADE
);
