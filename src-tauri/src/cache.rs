use crate::database::ClipboardRecord;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

// 剪切板记录内存缓存配置
const MAX_CLIPBOARD_CACHE_SIZE: usize = 1000;

#[derive(Debug, Clone)]
pub struct ClipboardCache {
    memory_cache: Arc<Mutex<LruCache<u64, ClipboardRecord>>>,
}

impl ClipboardCache {
    pub fn new() -> Self {
        let cache_size = NonZeroUsize::new(MAX_CLIPBOARD_CACHE_SIZE).unwrap();

        Self {
            memory_cache: Arc::new(Mutex::new(LruCache::new(cache_size))),
        }
    }

    // 添加记录到缓存
    pub fn add_record(&self, record: ClipboardRecord) {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        memory_cache.put(record.id, record);
    }

    // 从缓存中获取记录
    pub fn get_record(&self, id: u64) -> Option<ClipboardRecord> {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        memory_cache.get(&id).map(|r| r.clone())
    }

    // 获取所有缓存的记录
    pub fn get_all_records(&self) -> Vec<ClipboardRecord> {
        let memory_cache = self.memory_cache.lock().unwrap();
        let mut records: Vec<ClipboardRecord> = memory_cache.iter().map(|(_, v)| v.clone()).collect();
        // 按创建时间倒序排列
        records.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        records
    }

    // 删除记录
    pub fn delete_record(&self, id: u64) {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        memory_cache.pop(&id);
    }

    // 清空所有缓存
    pub fn clear(&self) {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        memory_cache.clear();
    }

    // 检查缓存中是否有数据
    pub fn has_data(&self) -> bool {
        let memory_cache = self.memory_cache.lock().unwrap();
        !memory_cache.is_empty()
    }
}
