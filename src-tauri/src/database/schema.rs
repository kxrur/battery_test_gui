// @generated automatically by Diesel CLI.

diesel::table! {
    battery_logs (record_id) {
        record_id -> Nullable<Integer>,
        id -> Integer,
        port -> Text,
        battery_temperature -> Integer,
        bench_temperature_mosfet -> Integer,
        bench_temperature_resistor -> Integer,
        load -> Integer,
        voltage -> Integer,
        current -> Integer,
        state -> Text,
        status -> Text,
        start_date -> Nullable<Text>,
        end_date -> Nullable<Text>,
        test_id -> Integer,
    }
}

diesel::table! {
    tests (test_id) {
        test_id -> Nullable<Integer>,
        test_name -> Text,
        start_date -> Text,
    }
}

diesel::joinable!(battery_logs -> tests (test_id));

diesel::allow_tables_to_appear_in_same_query!(
    battery_logs,
    tests,
);
