DROP TABLE formats;

ALTER TABLE files
    DROP COLUMN fileid,
    DROP COLUMN format_id,
    DROP COLUMN processed;
