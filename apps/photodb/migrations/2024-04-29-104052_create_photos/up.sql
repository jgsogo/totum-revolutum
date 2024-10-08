CREATE TABLE photos
(
    id     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    fileid BIGINT UNIQUE                     NOT NULL,
    path   TEXT                              NOT NULL
);
