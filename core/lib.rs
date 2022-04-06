pub const DB_PATH_NAME: &str = ".hosts";

fn db() -> sled::Db {
  // 获得当前用户的home目录
  let home_dir = dirs::home_dir().unwrap();
  // 拼接本地数据库存储路径
  let db_path = home_dir.join(crate::DB_PATH_NAME);
  // 返回数据库实例
  return sled::open(db_path).unwrap();
}

// 插入一条数据
pub fn insert(value: &&str) {
  match db().insert(value, vec![0]) {
    Ok(_) => {
      println!("添加成功✌️ {}", value);
    }
    Err(_) => {
      println!("添加失败👽 {}", value);
    }
  }
}

// 查找所有数据
pub fn find() -> Vec<String> {
  let mut values = vec![];
  let rows = db().iter();
  for row in rows {
    let (k, _) = row.unwrap();
    values.push(String::from_utf8_lossy(&k).to_string());
  }
  return values;
}
