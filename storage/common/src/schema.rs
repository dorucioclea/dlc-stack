use diesel::prelude::*;

table! {
    contracts (id) {
        id -> Int4,
        uuid -> Varchar,
        state -> Varchar,
        content -> Text,
    }
}
