-- Your SQL goes here
CREATE TABLE battery_logs (
            record_id INTEGER PRIMARY KEY AUTOINCREMENT,
            id INTEGER NOT NULL,
            port TEXT NOT NULL,
            temperature INTEGER NOT NULL,
            battery_temperature INTEGER NOT NULL,
            electronic_load_temperature INTEGER NOT NULL,
            voltage INTEGER NOT NULL,
            current INTEGER NOT NULL,
            state TEXT NOT NULL,
            status TEXT NOT NULL,
            start_date TEXT,
            end_date TEXT
        );
