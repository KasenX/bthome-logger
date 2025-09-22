CREATE TABLE IF NOT EXISTS samples (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    packet_counter INTEGER,
    temperature REAL,
    humidity REAL,
    battery INTEGER,
    ts DATETIME NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_samples_address ON samples (address);
CREATE INDEX IF NOT EXISTS idx_samples_ts ON samples (ts);
