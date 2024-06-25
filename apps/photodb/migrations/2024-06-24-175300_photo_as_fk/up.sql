CREATE TABLE photo_files
(
    file_id           INTEGER PRIMARY KEY NOT NULL,

    -- Fields that are interesting to our application
    fileid BIGINT UNIQUE NOT NULL, -- Identifier inside pcloud
    format_id INTEGER NOT NULL, -- the file format
    processed BOOLEAN NOT NULL DEFAULT 'FALSE', -- PhotoDB application did it's processing on this file

    --- Constraints
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (format_id) REFERENCES formats (id) ON DELETE SET NULL
);

CREATE TABLE video_files
(
    file_id           INTEGER PRIMARY KEY NOT NULL,

    -- Fields that are interesting to our application
    fileid BIGINT UNIQUE NOT NULL, -- Identifier inside pcloud
    format_id INTEGER NOT NULL, -- the file format
    processed BOOLEAN NOT NULL DEFAULT 'FALSE', -- PhotoDB application did it's processing on this file

    --- Constraints
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (format_id) REFERENCES formats (id) ON DELETE SET NULL
);
