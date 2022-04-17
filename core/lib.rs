pub const DB_PATH_NAME: &str = ".hosts";

pub fn db() -> sled::Db {
  // 获得当前用户的home目录
  let home_dir = dirs::home_dir().unwrap();
  // 拼接本地数据库存储路径
  let db_path = home_dir.join(crate::DB_PATH_NAME);
  // 创建数据库
  let db = sled::open(db_path).unwrap();
  // 设置合并函数
  db.set_merge_operator(concatenate_merge);
  // 返回数据库实例
  return db;
}

fn concatenate_merge(
  _key: &[u8],               // the key being merged
  old_value: Option<&[u8]>,  // the previous value, if one existed
  merged_bytes: &[u8]        // the new bytes being merged in
) -> Option<Vec<u8>> {       // set the new value, return None to delete
  let mut ret = old_value
    .map(|ov| ov.to_vec())
    .unwrap_or_else(|| vec![]);

  ret.extend_from_slice(merged_bytes);

  Some(ret)
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

pub fn save(key: &str, value: String) {
  match db().insert(key, value.to_string().into_bytes()) {
    Ok(_) => {
      println!("保存成功✌️ {}", value);
    }
    Err(_) => {
      println!("保存失败👽 {}", value);
    }
  }
}

pub fn merge(key: &str, values: Vec<String>) -> String {
  for (index, v) in values.iter().enumerate() {
    let res: sled::Result<Option<sled::IVec>>;
    let nv = (v.to_string() + "\n").into_bytes();
    if index == 0 {
      res = db().insert(key, nv);
    } else {
      res = db().merge(key, nv);
    }

    match res {
      Ok(_) => {}
      Err(_) => {}
    }
  }

  let value = db().get(key).unwrap().unwrap();
  
  // 清理保存的数据，不在乎是否成功
  match db().remove(key) {
    Ok(_) => {}
    Err(_) => {}
  }

  return String::from_utf8_lossy(&value).to_string();
}
