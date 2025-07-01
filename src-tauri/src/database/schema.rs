// @generated automatically by Diesel CLI.

diesel::table! {
    battery_logs (record_id) {
        record_id -> Nullable<Integer>,
        id -> Integer,
        port -> Text,
        temperature -> Integer,
        battery_temperature -> Integer,
        electronic_load_temperature -> Integer,
        voltage -> Integer,
        current -> Integer,
        state -> Text,
        status -> Text,
        start_date -> Nullable<Text>,
        end_date -> Nullable<Text>,
    }
}
