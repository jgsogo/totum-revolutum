CREATE TABLE photo_files
(
    file_id INTEGER PRIMARY KEY NOT NULL,

    -- Fields that are interesting to our application
    fileid BIGINT UNIQUE NOT NULL, -- Identifier inside pcloud
    format_id INTEGER NOT NULL, -- the file format
    processed BOOLEAN NOT NULL DEFAULT 'FALSE', -- PhotoDB application processed this file

    --- Constraints
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (format_id) REFERENCES formats (id) ON DELETE SET NULL
);

CREATE TABLE video_files
(
    file_id INTEGER PRIMARY KEY NOT NULL,

    -- Fields that are interesting to our application
    fileid BIGINT UNIQUE NOT NULL, -- Identifier inside pcloud
    format_id INTEGER NOT NULL, -- the file format
    processed BOOLEAN NOT NULL DEFAULT 'FALSE', -- PhotoDB application processed this file

    --- Constraints
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (format_id) REFERENCES formats (id) ON DELETE SET NULL
);

CREATE TABLE formats
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id INTEGER,

    format VARCHAR NOT NULL,

    FOREIGN KEY (parent_id) REFERENCES formats (id) ON DELETE CASCADE,
    UNIQUE (format) ON CONFLICT ABORT
);
INSERT INTO formats (id, format) VALUES (0, 'unknown');
INSERT INTO formats (id, format) VALUES (1, 'text');
INSERT INTO formats (id, format) VALUES (2, 'binary');
INSERT INTO formats (id, parent_id, format) VALUES (3, 2, 'image');
INSERT INTO formats (id, parent_id, format) VALUES (4, 2, 'video');
-- image formats
INSERT INTO formats (id, parent_id, format) VALUES (5, 3, 'png');
INSERT INTO formats (id, parent_id, format) VALUES (6, 3, 'jpeg');
INSERT INTO formats (id, parent_id, format) VALUES (7, 3, 'bmp');
-- text formats

-- video formats
