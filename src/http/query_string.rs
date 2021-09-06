use std::collections::HashMap;

pub struct QueryString<'buf> {
  params: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.params.get(key)
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut params = HashMap::new();
        for param_str in s.split('&') {
          let key: &str;
          let value: &str;
          if let Some(i) = param_str.find('=') {
            key = &param_str[..i];
            value = &param_str[i+1..];
          } else {
            key = param_str;
            value = "";
          }

          params.entry(key)
          .and_modify(|existing: &mut Value| match existing {
            Value::Single(v) => *existing = Value::Multiple(vec![v, value]),
            Value::Multiple(v) => v.push(value)
          })
          .or_insert(Value::Single(value));

        }
        
        Self{
          params
        }
    }
}