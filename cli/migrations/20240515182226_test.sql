CREATE TABLE test222 (
    version Int64,
    description Utf8 NOT NULL,
    checksum String NOT NULL,
    installed_on Timestamp NOT NULL,
    success Bool NOT NULL,
    execution_time Int64 NOT NULL,
    PRIMARY KEY (version)
);