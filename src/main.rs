use rand::prelude::*;
use rand::distributions::{Distribution, WeightedIndex, Uniform};
use chrono::{NaiveDate, Duration};
use std::env;

// 定義表格和欄位類別
const TABLES: &[&str] = &["users", "orders", "products", "customers"];

// 根據欄位類型分類
const STRING_COLUMNS: &[&str] = &["name", "status", "category"];
const NUMERIC_COLUMNS: &[&str] = &["id", "price", "quantity"];
const DATE_COLUMNS: &[&str] = &["created_at", "updated_at"];

const CONDITIONS: &[&str] = &["=", ">", "<", ">=", "<="];

/// 隨機生成 SQL 語句
fn generate_random_sql() -> String {
    let mut rng = thread_rng();
    
    // 定義 SQL 語句的類型和其相應的權重
    let sql_types = ["CREATE TABLE", "DROP TABLE", "ALTER TABLE", "INSERT", "DELETE", "UPDATE", "SELECT"];
    let weights = [1, 1, 3, 30, 5, 10, 50]; // 總和為 100

    // 使用 WeightedIndex 根據權重隨機選擇語句類型
    let dist = WeightedIndex::new(&weights).unwrap();
    let sql_type = sql_types[dist.sample(&mut rng)];

    match sql_type {
        "CREATE TABLE" => generate_create_table_sql(),
        "DROP TABLE" => generate_drop_table_sql(),
        "ALTER TABLE" => generate_alter_table_sql(),
        "INSERT" => generate_insert_sql(),
        "DELETE" => generate_delete_sql(),
        "UPDATE" => generate_update_sql(),
        "SELECT" => generate_select_sql(),
        _ => String::from(""),
    }
}

/// 隨機生成 SELECT 語句
fn generate_select_sql() -> String {
    let mut rng = thread_rng();
    let table = TABLES.choose(&mut rng).unwrap();
    
    let columns = [
        STRING_COLUMNS.choose(&mut rng),
        NUMERIC_COLUMNS.choose(&mut rng),
        DATE_COLUMNS.choose(&mut rng),
    ]
    .iter()
    .filter_map(|&col| col.cloned())
    .collect::<Vec<&str>>()
    .join(", ");
    
    // 多組合 WHERE 條件
    let conditions = generate_where_conditions(3 + rng.gen_range(0..3)); // 3 至 5 個條件

    format!("SELECT {} FROM {} WHERE {}", columns, table, conditions)
}

/// 隨機生成 INSERT 語句
fn generate_insert_sql() -> String {
    let mut rng = thread_rng();
    let table = TABLES.choose(&mut rng).unwrap();

    let columns = [
        STRING_COLUMNS.choose(&mut rng),
        NUMERIC_COLUMNS.choose(&mut rng),
        DATE_COLUMNS.choose(&mut rng),
    ]
    .iter()
    .filter_map(|&col| col.cloned())
    .collect::<Vec<&str>>()
    .join(", ");

    let values: Vec<String> = columns
        .split(", ")
        .map(|col| {
            if STRING_COLUMNS.contains(&col) {
                format!("'{}'", generate_random_string())
            } else if NUMERIC_COLUMNS.contains(&col) {
                rng.gen_range(1..100).to_string()
            } else if DATE_COLUMNS.contains(&col) {
                generate_oracle_date()
            } else {
                "NULL".to_string()
            }
        })
        .collect();

    format!("INSERT INTO {} ({}) VALUES ({})", table, columns, values.join(", "))
}

/// 隨機生成 UPDATE 語句
fn generate_update_sql() -> String {
    let mut rng = thread_rng();
    let table = TABLES.choose(&mut rng).unwrap();
    let column = [
        STRING_COLUMNS.choose(&mut rng),
        NUMERIC_COLUMNS.choose(&mut rng),
        DATE_COLUMNS.choose(&mut rng),
    ]
    .iter()
    .filter_map(|&col| col.cloned())
    .choose(&mut rng)
    .unwrap();

    let value = match column {
        col if STRING_COLUMNS.contains(&col) => format!("'{}'", generate_random_string()),
        col if NUMERIC_COLUMNS.contains(&col) => rng.gen_range(1..100).to_string(),
        col if DATE_COLUMNS.contains(&col) => generate_oracle_date(),
        _ => String::from("NULL"),
    };

    let conditions = generate_where_conditions(3 + rng.gen_range(0..3)); // 3 至 5 個條件

    format!("UPDATE {} SET {} = {} WHERE {}", table, column, value, conditions)
}

/// 隨機生成 DELETE 語句
fn generate_delete_sql() -> String {
    let mut rng = thread_rng();
    let table = TABLES.choose(&mut rng).unwrap();
    
    let conditions = generate_where_conditions(3 + rng.gen_range(0..3)); // 3 至 5 個條件

    format!("DELETE FROM {} WHERE {}", table, conditions)
}

/// 隨機生成 CREATE TABLE 語句
fn generate_create_table_sql() -> String {
    let mut rng = thread_rng();
    let table_name = TABLES.choose(&mut rng).unwrap();
    let columns = vec![
        format!("id NUMBER PRIMARY KEY"),
        format!("name VARCHAR2(50)"),
        format!("price NUMBER"),
        format!("quantity NUMBER"),
        format!("created_at DATE"),
    ];

    format!("CREATE TABLE {} (\n    {}\n)", table_name, columns.join(",\n    "))
}

/// 隨機生成 ALTER TABLE 語句
fn generate_alter_table_sql() -> String {
    let mut rng = thread_rng();
    let table_name = TABLES.choose(&mut rng).unwrap();
    let column_name = STRING_COLUMNS.choose(&mut rng).unwrap();
    format!("ALTER TABLE {} ADD {} VARCHAR2(100)", table_name, column_name)
}

/// 隨機生成 DROP TABLE 語句
fn generate_drop_table_sql() -> String {
    let mut rng = thread_rng();
    let table_name = TABLES.choose(&mut rng).unwrap();
    format!("DROP TABLE {}", table_name)
}

/// 生成多組合 WHERE 條件
fn generate_where_conditions(num_conditions: usize) -> String {
    let mut rng = thread_rng();
    let mut conditions = Vec::new();

    for _ in 0..num_conditions {
        let column = [
            STRING_COLUMNS.choose(&mut rng),
            NUMERIC_COLUMNS.choose(&mut rng),
            DATE_COLUMNS.choose(&mut rng),
        ]
        .iter()
        .filter_map(|&col| col.cloned())
        .choose(&mut rng)
        .unwrap();

        let condition = if STRING_COLUMNS.contains(&column) {
            let values = (0..rng.gen_range(2..=10))
                .map(|_| format!("'{}'", generate_random_string()))
                .collect::<Vec<String>>()
                .join(", ");
            format!("{} IN ({})", column, values)
        } else if NUMERIC_COLUMNS.contains(&column) {
            let op = CONDITIONS.choose(&mut rng).unwrap();
            format!("{} {} {}", column, op, rng.gen_range(1..100))
        } else if DATE_COLUMNS.contains(&column) {
            format!(
                "{} BETWEEN {} AND {}",
                column,
                generate_oracle_date(),
                generate_oracle_date()
            )
        } else {
            String::new()
        };

        conditions.push(condition);
    }

    conditions.join(" AND ")
}

/// 隨機生成字串
fn generate_random_string() -> String {
    let mut rng = thread_rng();
    let char_range = Uniform::from(b'a'..=b'z');
    let chars: Vec<char> = (0..5).map(|_| char_range.sample(&mut rng) as char).collect();
    chars.into_iter().collect()
}

/// 隨機生成符合 Oracle 的日期格式
fn generate_oracle_date() -> String {
    let start_date = NaiveDate::from_ymd(2020, 1, 1);
    let days_offset: i64 = rand::thread_rng().gen_range(0..=365 * 3);
    let date = start_date + Duration::days(days_offset);
    format!("TO_DATE('{}', 'YYYY-MM-DD')", date.format("%Y-%m-%d"))
}

fn main() {
    // 取得命令列參數，若無參數則預設為 10
    let args: Vec<String> = env::args().collect();
    let num_sql = args.get(1).and_then(|n| n.parse::<usize>().ok()).unwrap_or(10);

    // 隨機生成指定數量的 SQL 語句並顯示
    for _ in 0..num_sql {
        println!("{}", generate_random_sql());
    }
}

