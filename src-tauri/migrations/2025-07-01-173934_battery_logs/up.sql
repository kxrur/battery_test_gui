-- Your SQL goes here
CREATE TABLE tests (
    test_id INTEGER PRIMARY KEY AUTOINCREMENT,
    test_name TEXT NOT NULL,
    start_date TEXT NOT NULL
);
CREATE TABLE battery_logs (
    record_id integer primary key autoincrement, id integer not null,
    port TEXT NOT NULL,
    temperature INTEGER NOT NULL,
    battery_temperature INTEGER NOT NULL,
    electronic_load_temperature INTEGER NOT NULL,
    voltage INTEGER NOT NULL,
    current INTEGER NOT NULL,
    state TEXT NOT NULL,
    status TEXT NOT NULL,
    start_date TEXT,
    end_date TEXT,
    test_id INTEGER NOT NULL,
    FOREIGN KEY (test_id) REFERENCES tests(test_id) ON DELETE CASCADE
);
