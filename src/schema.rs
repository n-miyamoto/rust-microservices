table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    sensor_data (id) {
        id -> Int4,
        create_at -> Timestamptz,
        data0 -> Nullable<Float4>,
        data1 -> Nullable<Float4>,
        data2 -> Nullable<Float4>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    sensor_data,
);
