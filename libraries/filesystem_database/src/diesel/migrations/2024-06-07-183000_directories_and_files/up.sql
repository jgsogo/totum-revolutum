-- Directories
CREATE TABLE directories
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name      VARCHAR NOT NULL,
    parent_id INTEGER,
    FOREIGN KEY (parent_id) REFERENCES directories (id)
);

INSERT INTO directories (id, name) VALUES (0, '/');

-- Files
CREATE TABLE files
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name         VARCHAR NOT NULL,
    directory_id INTEGER NOT NULL,

    --- Some properties (the ones required by FileMetadata)
    -- hash
    -- size

    --- Constraints
    UNIQUE (name, directory_id) ON CONFLICT ABORT
);
