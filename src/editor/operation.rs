// 生成md表格源码
fn generate_table_code(rows: usize, cols: usize) -> String {
    let mut table = String::new();
    // 生成表头
    for i in 1..=cols {
        table.push_str(&format!("| Column{} ", i));
    }
    table.push_str("|\n");
    // 生成分隔行
    for _ in 1..=cols {
        table.push_str("|---------");
    }
    table.push_str("|\n");
    // 生成数据行
    for _ in 1..=rows {
        for _ in 1..=cols {
            table.push_str("| Cell   ");
        }
        table.push_str("|\n");
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table() {
        let table = generate_table_code(7, 5);
        println!("{}", table);
    }
}
