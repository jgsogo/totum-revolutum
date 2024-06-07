CREATE TABLE m2m_posts_tags
(
    post_id  INT NOT NULL,
    tag VARCHAR NOT NULL,

    FOREIGN KEY (post_id) REFERENCES posts (id),
    FOREIGN KEY (tag) REFERENCES tags (tag),
    PRIMARY KEY (post_id, tag)
);
