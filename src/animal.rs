#[napi(constructor)]
pub struct Animal {
  pub name: String,
  pub age: i32,
}

#[napi]
impl Animal {
  #[napi]
  pub fn get_name(&self) -> napi::Result<String> {
    Ok(self.name.clone())
  }

  #[napi]
  pub async fn get_age(&self) -> napi::Result<i32> {
    Ok(self.age)
  }
}
