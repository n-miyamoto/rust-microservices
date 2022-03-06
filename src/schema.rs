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
        writekey -> Text,
        create_at -> Timestamptz,
        d1 -> Float4,
        d2 -> Float4,
        d3 -> Float4,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    sensor_data,
);
