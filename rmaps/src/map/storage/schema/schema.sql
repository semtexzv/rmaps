CREATE TABLE IF NOT EXISTS resources (
  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  url             TEXT    NOT NULL,
  kind            INTEGER NOT NULL,
  data            BLOB,
  expires         INTEGER,
  accessed        INTEGER NOT NULL,
  UNIQUE (url)
);

CREATE INDEX IF NOT EXISTS resources_accessed  ON resources (accessed);
