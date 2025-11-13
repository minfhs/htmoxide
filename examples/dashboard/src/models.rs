/// User data model
#[derive(Clone, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: String,
}
