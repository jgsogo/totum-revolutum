-- A temporal table so we can add some additional constraints.
CREATE TABLE files_tmp
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name         VARCHAR NOT NULL,
    directory_id INTEGER NOT NULL,

    -- Fields required so it can be used to index a filesystem
    hash VARCHAR NOT NULL ,
    size INTEGER NOT NULL,

    -- Fields that are interesting to our application
    fileid BIGINT UNIQUE, -- Identifier inside pcloud
    format_id INTEGER, -- the file format
    processed BOOLEAN NOT NULL DEFAULT 'FALSE', -- PhotoDB application did it's processing on this file

    --- Constraints
    FOREIGN KEY (directory_id) REFERENCES directories (id) ON DELETE CASCADE,
    FOREIGN KEY (format_id) REFERENCES formats (id) ON DELETE SET NULL,
    UNIQUE (name, directory_id) ON CONFLICT ABORT
);


CREATE TABLE formats
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id   INTEGER,

    format         VARCHAR NOT NULL,

    FOREIGN KEY (parent_id) REFERENCES formats (id) ON DELETE CASCADE,
    UNIQUE (format) ON CONFLICT ABORT
);
INSERT INTO formats (id, format) VALUES (0, 'unknown');
INSERT INTO formats (id, format) VALUES (1, 'text');
INSERT INTO formats (id, format) VALUES (2, 'binary');
INSERT INTO formats (id, parent_id, format) VALUES (3, 2, 'image');
INSERT INTO formats (id, parent_id, format) VALUES (4, 2, 'video');


-- Sqlite3 doesn't allow ALTER TABLE to insert constraints (like the ones we want to add for `format_id`. The
-- workaround is to create a new table with the desired constraints, copy everything there and remove the original one.
INSERT INTO files_tmp (id, name, directory_id, hash, size)
    SELECT id, name, directory_id, hash, size
    FROM files;

DROP TABLE files;

ALTER TABLE files_tmp RENAME TO `files`;
