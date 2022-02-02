CREATE TABLE items
(
    id     BIGINT        NOT NULL PRIMARY KEY,
    parent BIGINT        NOT NULL,
    name   VARCHAR(1024) NOT NULL,
    size   BIGINT        NOT NULL
);