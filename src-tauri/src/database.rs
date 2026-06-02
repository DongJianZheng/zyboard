use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardRecord {
    pub id: u64,
    pub content: String,
    pub content_preview: Option<String>,
    pub data_type: String,
    pub is_favorite: bool,
    pub create_time: u64,
}

pub struct ClipboardDB {
    pub conn: Connection,
}

impl ClipboardDB {
    pub fn new(app_handle: &AppHandle, custom_data_dir: Option<String>) -> Result<Self, String> {
        let path = if let Some(custom_dir) = custom_data_dir {
            // 使用自定义目录
            let custom_path = std::path::PathBuf::from(custom_dir);
            if !custom_path.exists() {
                std::fs::create_dir_all(&custom_path)
                    .map_err(|e| format!("创建自定义目录失败: {}", e))?;
            }
            custom_path
        } else {
            // 使用默认的应用数据目录
            app_handle
                .path()
                .app_data_dir()
                .map_err(|e| format!("获取应用数据目录失败: {}", e))?
        };

        let db_path = path.join("clipboard.db");

        // 确保目录存在
        if !db_path.exists() {
            std::fs::create_dir_all(&path)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }

        let conn = Connection::open(db_path)
            .map_err(|e| format!("打开数据库失败: {}", e))?;

        let db = Self { conn };
        db.init_table()?;
        Ok(db)
    }

    fn init_table(&self) -> Result<(), String> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS clipboard_records (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    content TEXT NOT NULL,
                    content_preview TEXT,
                    data_type TEXT NOT NULL,
                    is_favorite INTEGER DEFAULT 0,
                    create_time INTEGER NOT NULL
                )",
                [],
            )
            .map_err(|e| format!("创建表失败: {}", e))?;

        // 创建索引
        self.conn
            .execute(
                "CREATE INDEX IF NOT EXISTS idx_create_time ON clipboard_records(create_time DESC)",
                [],
            )
            .map_err(|e| format!("创建索引失败: {}", e))?;

        Ok(())
    }

    pub fn insert_if_not_exist(&self, record: ClipboardRecord) -> Result<Option<u64>, String> {
        // 检查是否已存在相同内容
        let exists: bool = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM clipboard_records WHERE content = ?1",
                params![record.content],
                |row| row.get(0),
            )
            .unwrap_or(0) > 0;

        if exists {
            return Ok(None);
        }

        self.conn
            .execute(
                "INSERT INTO clipboard_records (content, content_preview, data_type, is_favorite, create_time)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    record.content,
                    record.content_preview,
                    record.data_type,
                    if record.is_favorite { 1 } else { 0 },
                    record.create_time,
                ],
            )
            .map_err(|e| format!("插入记录失败: {}", e))?;

        let id = self.conn.last_insert_rowid() as u64;
        Ok(Some(id))
    }

    pub fn insert(&self, record: ClipboardRecord) -> Result<u64, String> {
        self.conn
            .execute(
                "INSERT INTO clipboard_records (content, content_preview, data_type, is_favorite, create_time)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    record.content,
                    record.content_preview,
                    record.data_type,
                    if record.is_favorite { 1 } else { 0 },
                    record.create_time,
                ],
            )
            .map_err(|e| format!("插入记录失败: {}", e))?;

        let id = self.conn.last_insert_rowid() as u64;
        Ok(id)
    }

    pub fn get_all_records(&self) -> Result<Vec<ClipboardRecord>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, content, content_preview, data_type, is_favorite, create_time
                 FROM clipboard_records
                 ORDER BY create_time DESC",
            )
            .map_err(|e| format!("准备查询失败: {}", e))?;

        let records = stmt
            .query_map([], |row| {
                Ok(ClipboardRecord {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    content_preview: row.get(2)?,
                    data_type: row.get(3)?,
                    is_favorite: row.get::<_, i32>(4)? == 1,
                    create_time: row.get(5)?,
                })
            })
            .map_err(|e| format!("查询记录失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("解析记录失败: {}", e))?;

        Ok(records)
    }

    pub fn delete_by_id(&self, id: u64) -> Result<bool, String> {
        let rows = self
            .conn
            .execute("DELETE FROM clipboard_records WHERE id = ?1", params![id])
            .map_err(|e| format!("删除记录失败: {}", e))?;

        Ok(rows > 0)
    }

    pub fn clear_all(&self) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM clipboard_records", [])
            .map_err(|e| format!("清空记录失败: {}", e))?;

        Ok(())
    }

    pub fn delete_over_limit(&self, limit: usize) -> Result<bool, String> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM clipboard_records", [], |row| {
                row.get(0)
            })
            .unwrap_or(0);

        println!("📊 [delete_over_limit] 当前记录数: {}, 限制: {}", count, limit);

        if count as usize > limit {
            println!("🗑️ [delete_over_limit] 记录数超过限制，准备删除旧记录");

            // 获取要保留的记录ID
            let mut stmt = self
                .conn
                .prepare(
                    "SELECT id FROM clipboard_records ORDER BY create_time DESC LIMIT ?1",
                )
                .map_err(|e| format!("准备查询失败: {}", e))?;

            let keep_ids: Vec<u64> = stmt
                .query_map(params![limit], |row| row.get(0))
                .map_err(|e| format!("查询记录失败: {}", e))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("解析记录失败: {}", e))?;

            println!("✅ [delete_over_limit] 要保留的记录ID: {:?}", keep_ids);

            // 删除不在保留列表中的记录
            if !keep_ids.is_empty() {
                let placeholders = keep_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                let ids: Vec<i64> = keep_ids.iter().map(|&id| id as i64).collect();

                let query = format!(
                    "DELETE FROM clipboard_records WHERE id NOT IN ({})",
                    placeholders
                );

                let deleted = self.conn
                    .execute(&query, rusqlite::params_from_iter(ids))
                    .map_err(|e| format!("删除超额记录失败: {}", e))?;

                println!("🗑️ [delete_over_limit] 已删除 {} 条记录", deleted);

                // 删除后刷新缓存
                let new_count: i64 = self
                    .conn
                    .query_row("SELECT COUNT(*) FROM clipboard_records", [], |row| {
                        row.get(0)
                    })
                    .unwrap_or(0);
                println!("📊 [delete_over_limit] 删除后记录数: {}", new_count);

                return Ok(true);
            }
        }

        println!("✅ [delete_over_limit] 记录数未超过限制，无需删除");
        Ok(false)
    }
}
