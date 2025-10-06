PRAGMA foreign_keys = ON;
-- "INTEGER PRIMARY KEY" means ROWID, which is a usable primary key for sqlite
create table lagerplatz
(
    id INTEGER PRIMARY KEY not null,
    ort        TEXT not null,
    typ TEXT not null
);

create table kategorien
(
    id INTEGER PRIMARY KEY not null,
    comment TEXT not null
);


create table lagerbelegung
(
    id                    INTEGER PRIMARY KEY not null,
    beschreibung          TEXT                  not null,
    einlagerungszeitpunkt datetime              not null,
    can_be_outside        boolean,
    kategorie_id             INTEGER                  not null,
    lagerplatz_id         INTEGER                  not null,
    FOREIGN KEY (kategorie_id) REFERENCES kategorien (id),
    FOREIGN KEY (lagerplatz_id) REFERENCES lagerplatz (id)
);

create table lagertransaktionen
(
    id INTEGER PRIMARY KEY not null,
    lagerbelegung_id INTEGER not null,
    artikeldelta integer not null,
    datum datetile not null,
    FOREIGN KEY (lagerbelegung_id) REFERENCES lagerbelegung (id)
);
