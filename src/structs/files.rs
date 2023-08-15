use axum::extract::multipart::Field;

pub struct File<'a> {
    pub name: String,
    pub content_type: String,
    pub field: Field<'a>,
}