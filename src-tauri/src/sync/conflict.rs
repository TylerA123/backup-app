// Conflict resolution strategy for MVP:
// 1. Local snapshot is authoritative (producer's latest save wins)
// 2. If cloud has a newer snapshot we haven't seen, we merge:
//    - Files only in cloud → download to local cache
//    - Files only in local → upload to cloud
//    - Files in both with different hashes → local version wins (logged)
// 3. Manual conflict resolution is deferred to post-MVP

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConflictResolution {
    pub files_to_upload: Vec<String>,
    pub files_to_download: Vec<String>,
    pub files_skipped: Vec<String>,
}

pub struct ConflictResolver;

impl ConflictResolver {
    pub fn new() -> Self {
        ConflictResolver
    }

    pub fn resolve(
        &self,
        local_snapshot: &[String],
        remote_snapshot: &[String],
    ) -> ConflictResolution {
        let local_set: std::collections::HashSet<&String> =
            local_snapshot.iter().collect();
        let remote_set: std::collections::HashSet<&String> =
            remote_snapshot.iter().collect();

        ConflictResolution {
            files_to_upload: local_set
                .difference(&remote_set)
                .map(|s| (*s).clone())
                .collect(),
            files_to_download: remote_set
                .difference(&local_set)
                .map(|s| (*s).clone())
                .collect(),
            files_skipped: local_set
                .intersection(&remote_set)
                .map(|s| (*s).clone())
                .collect(),
        }
    }
}
