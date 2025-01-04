diesel::table! {
    vertices{
        id -> Integer,
        payload -> Text,

    }
}
diesel::table! {
    edges(start, finish){
        start -> Integer,
        finish -> Integer,
        payload -> Text,

    }
}
