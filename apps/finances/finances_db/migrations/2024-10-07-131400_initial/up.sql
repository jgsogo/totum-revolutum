CREATE TABLE data_accountholder
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR UNIQUE NOT NULL,
    owner INTEGER NOT NULL
);

CREATE TABLE data_accounttype
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE data_account
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    identifier VARCHAR UNIQUE,
    name TEXT UNIQUE NOT NULL,
    is_numerable BOOLEAN NOT NULL DEFAULT 'FALSE',
    ccy VARCHAR NOT NULL,
    open DATE NOT NULL,
    close DATE,

    holder_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,

    FOREIGN KEY (holder_id)
    REFERENCES data_accountholder (id) ON DELETE CASCADE,

    FOREIGN KEY (type_id) REFERENCES data_accounttype (id) ON DELETE RESTRICT
);

CREATE TABLE data_snapshot
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    amount DECIMAL(14, 4) NOT NULL,
    quantity INTEGER,
    unit_value DECIMAL(14, 4),

    date_value DATE NOT NULL,
    account_id INTEGER NOT NULL,

    FOREIGN KEY (account_id) REFERENCES data_account (id) ON DELETE RESTRICT,
    CONSTRAINT quantity_positive CHECK (quantity > 0),
    UNIQUE (date_value, account_id) ON CONFLICT ABORT
);

CREATE TABLE data_movementtype
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    name VARCHAR UNIQUE NOT NULL,
    level SMALLINT NOT NULL,
    parent_id INTEGER,

    FOREIGN KEY (parent_id) REFERENCES data_movementtype (id) ON DELETE SET NULL,
    CONSTRAINT level_positive CHECK (level > 0)
);

CREATE TABLE data_movement
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    amount DECIMAL(14, 4) NOT NULL,
    quantity INTEGER,
    unit_value DECIMAL(14, 4),

    date_value DATE NOT NULL,
    account_id INTEGER NOT NULL,

    direction INTEGER NOT NULL,
    "date" DATE NOT NULL,

    fx_id INTEGER,
    transfer_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,

    FOREIGN KEY (account_id) REFERENCES data_account (id) ON DELETE RESTRICT,
    FOREIGN KEY (fx_id) REFERENCES data_fx (id) ON DELETE RESTRICT,
    FOREIGN KEY (transfer_id) REFERENCES data_transfer (id) ON DELETE CASCADE,
    FOREIGN KEY (type_id) REFERENCES data_movementtype (id) ON DELETE RESTRICT
);


CREATE TABLE data_fx
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    "foreign" VARCHAR NOT NULL,
    "local" VARCHAR NOT NULL,

    rate DECIMAL(14, 4) NOT NULL,
    date_value DATE NOT NULL
);

CREATE TABLE data_transfer
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    description TEXT NOT NULL
);
