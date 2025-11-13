PRAGMA foreign_keys = ON;
-- "INTEGER PRIMARY KEY" means ROWID, which is a usable primary key for sqlite
create table storage_boxes
(
    id        INTEGER PRIMARY KEY not null,
    place     TEXT                not null,
    item_type INTEGER             not null,
    FOREIGN KEY (item_type) REFERENCES item_types (id)
);

create table categories
(
    id      INTEGER PRIMARY KEY not null,
    comment TEXT                not null
);


create table allocations
(
    id             INTEGER PRIMARY KEY not null,
    description    TEXT                not null,
    date_of_entry  datetime            not null,
    can_be_outside boolean,
    category_id    INTEGER             not null,
    storage_box_id INTEGER             not null,
    FOREIGN KEY (category_id) REFERENCES categories (id),
    FOREIGN KEY (storage_box_id) REFERENCES storage_boxes (id)
);

create table transactions
(
    id            INTEGER PRIMARY KEY not null,
    allocation_id INTEGER             not null,
    item_delta    integer             not null,
    date          datetime            not null,
    FOREIGN KEY (allocation_id) REFERENCES allocations (id)
);

create table item_types
(
    id               INTEGER PRIMARY KEY not null,
    storage_property TEXT                not null
);
