use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

/// Represents a source file with content and metadata
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub content: String,
    pub path: String,
    pub lines: Vec<String>,
}

impl SourceFile {
    /// Create a SourceFile from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path_str))?;
        
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        Ok(SourceFile {
            content,
            path: path_str,
            lines,
        })
    }
    
    /// Create a SourceFile from a string (useful for testing)
    pub fn from_string(content: String, path: String) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        SourceFile {
            content,
            path,
            lines,
        }
    }
    
    /// Get a specific line by line number (1-indexed)
    pub fn get_line(&self, line_number: usize) -> Option<&str> {
        self.lines.get(line_number.saturating_sub(1)).map(|s| s.as_str())
    }
    
    /// Get the total number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    /// Get a range of lines (1-indexed, inclusive)
    pub fn get_lines(&self, start: usize, end: usize) -> Vec<&str> {
        let start_idx = start.saturating_sub(1);
        let end_idx = end;  // end is inclusive, so we don't subtract 1
        
        if start_idx >= self.lines.len() || start > end {
            return Vec::new();
        }
        
        let actual_end = end_idx.min(self.lines.len());
        self.lines[start_idx..actual_end].iter().map(|s| s.as_str()).collect()
    }
}

/// Read a source file from the filesystem
/// 
/// This is the main entry point for the Input Stage of the compilation pipeline.
pub fn read_source<P: AsRef<Path>>(path: P) -> Result<SourceFile> {
    SourceFile::from_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_read_source_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "CONTEXT Test {{").unwrap();
        writeln!(temp_file, "  VAR User").unwrap();
        writeln!(temp_file, "}}").unwrap();
        
        let source = read_source(temp_file.path()).unwrap();
        assert_eq!(source.lines.len(), 3);
        assert!(source.content.contains("CONTEXT Test"));
        assert_eq!(source.get_line(1), Some("CONTEXT Test {"));
        assert_eq!(source.get_line(2), Some("  VAR User"));
        assert_eq!(source.get_line(3), Some("}"));
    }
    
    #[test]
    fn test_source_file_from_string() {
        let content = "CONTEXT Test {\n  VAR User\n}".to_string();
        let source = SourceFile::from_string(content, "test.navi".to_string());
        
        assert_eq!(source.line_count(), 3);
        assert_eq!(source.path, "test.navi");
        assert_eq!(source.get_line(2), Some("  VAR User"));
    }
    
    #[test]
    fn test_get_lines_range() {
        let content = "line1\nline2\nline3\nline4\nline5".to_string();
        let source = SourceFile::from_string(content, "test.navi".to_string());
        
        let lines = source.get_lines(2, 4);
        assert_eq!(lines, vec!["line2", "line3", "line4"]);
        
        // Test edge cases
        let lines = source.get_lines(1, 1);
        assert_eq!(lines, vec!["line1"]);  // Should return the first line
        
        let lines = source.get_lines(4, 10);
        assert_eq!(lines, vec!["line4", "line5"]);
    }
}
