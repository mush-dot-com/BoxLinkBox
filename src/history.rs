use crate::domain::Library;
use std::collections::VecDeque;
use std::io::Write;

pub struct History {
    entries: VecDeque<(Library, usize)>,
    bytes: usize,
    byte_limit: usize,
}
impl Default for History {
    fn default() -> Self {
        Self::with_limit(32 * 1024 * 1024)
    }
}
impl History {
    pub fn with_limit(byte_limit: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            bytes: 0,
            byte_limit,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn last(&self) -> Option<&Library> {
        self.entries.back().map(|(data, _)| data)
    }
    pub fn pop(&mut self) {
        if let Some((_, bytes)) = self.entries.pop_back() {
            self.bytes -= bytes;
        }
    }
    pub fn push(&mut self, data: Library) {
        // Count the serialized size without allocating another copy of all data.
        let mut counter = Counter(0);
        if serde_json::to_writer(&mut counter, &data).is_err() || counter.0 > self.byte_limit {
            // Do not let undo jump across a state that could not be retained.
            self.entries.clear();
            self.bytes = 0;
            return;
        }
        while self.entries.len() >= 30 || self.bytes + counter.0 > self.byte_limit {
            if let Some((_, bytes)) = self.entries.pop_front() {
                self.bytes -= bytes;
            }
        }
        self.bytes += counter.0;
        self.entries.push_back((data, counter.0));
    }
}
struct Counter(usize);
impl Write for Counter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.0 = self.0.saturating_add(buffer.len());
        Ok(buffer.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
