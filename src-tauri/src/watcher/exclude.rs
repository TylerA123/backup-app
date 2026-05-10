pub struct ExclusionRules {
    patterns: Vec<String>,
}

impl ExclusionRules {
    pub fn new() -> Self {
        let patterns = vec![
            // System files
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
            ".Spotlight-V100".to_string(),
            ".Trashes".to_string(),
            // Hidden directories
            ".git".to_string(),
            ".svn".to_string(),
            ".studiobackup".to_string(),
            // Temporary files
            "*.tmp".to_string(),
            "*.temp".to_string(),
            "~*".to_string(),
            // Cache directories
            "__macosx".to_string(),
            // VST/AU cache files
            "*.vstcache".to_string(),
            // Project specific - Ableton
            "*.asd".to_string(), // Ableton analysis files (re-buildable)
            // Project specific - Logic
            "*.lsof".to_string(), // Logic temporary files
            // Project specific - FL Studio
            "*.fpl".to_string(), // FL Piano roll (re-buildable)
            // Project specific - Cubase
            "*.nib".to_string(), // Cubase GUI cache
        ];
        ExclusionRules { patterns }
    }

    pub fn is_excluded(&self, path: &str) -> bool {
        let lower = path.to_lowercase();
        for pattern in &self.patterns {
            if pattern.starts_with('*') {
                if lower.ends_with(&pattern[1..]) {
                    return true;
                }
            } else if pattern.starts_with('~') {
                if let Some(name) = std::path::Path::new(path).file_name() {
                    if name.to_string_lossy().starts_with('~') {
                        return true;
                    }
                }
            } else {
                if lower.contains(&pattern.to_lowercase()) {
                    return true;
                }
            }
        }
        false
    }
}
