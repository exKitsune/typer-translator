CREATE TABLE IF NOT EXISTS whitelist (
	id INTEGER PRIMARY KEY,
	process TEXT
);

CREATE TABLE IF NOT EXISTS saved (
	id INTEGER PRIMARY KEY,
	phrase TEXT,
    tl_phrase TEXT
);