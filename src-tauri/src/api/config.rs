pub struct AppConfig {
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub b2_key_id: String,
    pub b2_app_key: String,
    pub b2_bucket: String,
    pub b2_endpoint: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        // In production, these come from Tauri plugin-store (persisted settings)
        // For now, hardcoded for development
        AppConfig {
            supabase_url: "https://oewsafpfhkteofylqkab.supabase.co".to_string(),
            supabase_anon_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im9ld3NhZnBmaGt0ZW9meWxxa2FiIiwicm9sZSI6ImFub24iLCJpYXQiOjE3Nzg0MjY3MjAsImV4cCI6MjA5NDAwMjcyMH0.d1oze-iPoq-quzX_fPZzcFks9nebyPJskg6OzUAChls".to_string(),
            b2_key_id: "0033cfd6b809cc40000000001".to_string(),
            b2_app_key: "K003kdX1K2CXQw+GWFo8ppbtxlIr0Es".to_string(),
            b2_bucket: "backapp".to_string(),
            b2_endpoint: "https://s3.eu-central-003.backblazeb2.com".to_string(),
        }
    }
}
