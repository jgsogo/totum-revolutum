-- Directories
CREATE TABLE directories
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id INTEGER,

    -- Full path (relative to root)
    full_path      VARCHAR NOT NULL,

    FOREIGN KEY (parent_id) REFERENCES directories (id) ON DELETE CASCADE,
    UNIQUE (full_path) ON CONFLICT ABORT
);

INSERT INTO directories (id, full_path) VALUES (0, '');

-- Files
CREATE TABLE files
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name         VARCHAR NOT NULL,
    directory_id INTEGER NOT NULL,

    -- Computed hash of the file contents
    hash VARCHAR NOT NULL ,
    -- Size of the file
    size INTEGER NOT NULL,

    --- Constraints
    FOREIGN KEY (directory_id) REFERENCES directories (id) ON DELETE CASCADE,
    UNIQUE (name, directory_id) ON CONFLICT ABORT
);
