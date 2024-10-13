CREATE TABLE tags
(
    tag VARCHAR PRIMARY KEY NOT NULL,
    parent VARCHAR,
    FOREIGN KEY (parent) REFERENCES tags (tag)
);
