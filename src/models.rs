#[derive(Queryable,Debug)]
pub struct Book{
  pub id:i32,
  pub title:String,
  pub author:String,
  pub published:bool
}