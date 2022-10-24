// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "productstatus"))]
    pub struct Productstatus;
}

diesel::table! {
    group (name) {
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Productstatus;

    products (id) {
        id -> Uuid,
        url -> Varchar,
        status -> Nullable<Productstatus>,
        sales -> Nullable<Int4>,
        sellerid -> Nullable<Uuid>,
    }
}

diesel::table! {
    sellers (id) {
        id -> Uuid,
        url -> Varchar,
        lastyearsales -> Nullable<Int4>,
        reputation -> Nullable<Int4>,
    }
}

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

diesel::joinable!(products -> sellers (sellerid));

diesel::allow_tables_to_appear_in_same_query!(
    group,
    products,
    sellers,
    spatial_ref_sys,
);
