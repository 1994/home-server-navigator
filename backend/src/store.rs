use crate::models::ServiceEntry;
use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Clone)]
pub struct ServiceStore {
    path: PathBuf,
    backup_path: PathBuf,
}

impl ServiceStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let backup_path = PathBuf::from(format!("{}.bak", path.display()));
        Self { path, backup_path }
    }

    pub async fn load_services(&self) -> Result<Vec<ServiceEntry>> {
        if !self.path.exists() {
            self.ensure_parent_dir().await?;
            return Ok(Vec::new());
        }

        match fs::read_to_string(&self.path).await {
            Ok(content) => self.parse_content(&content),
            Err(error) => {
                Err(error).with_context(|| format!("failed reading {}", self.path.display()))
            }
        }
    }

    pub async fn save_services(&self, services: &[ServiceEntry]) -> Result<()> {
        self.ensure_parent_dir().await?;
        let json =
            serde_json::to_string_pretty(services).context("failed to serialize services")?;

        if self.path.exists() {
            let _ = fs::copy(&self.path, &self.backup_path).await;
        }

        let temp_path = PathBuf::from(format!("{}.tmp", self.path.display()));
        {
            let mut file = fs::File::create(&temp_path)
                .await
                .with_context(|| format!("failed creating temp file {}", temp_path.display()))?;
            file.write_all(json.as_bytes())
                .await
                .context("failed writing services temp content")?;
            file.flush()
                .await
                .context("failed flushing services temp file")?;
        }

        fs::rename(&temp_path, &self.path).await.with_context(|| {
            format!(
                "failed renaming {} to {}",
                temp_path.display(),
                self.path.display()
            )
        })?;

        Ok(())
    }

    fn parse_content(&self, content: &str) -> Result<Vec<ServiceEntry>> {
        match serde_json::from_str::<Vec<ServiceEntry>>(content) {
            Ok(services) => Ok(services),
            Err(primary_error) => {
                if self.backup_path.exists() {
                    let backup_content =
                        std::fs::read_to_string(&self.backup_path).with_context(|| {
                            format!("failed reading backup {}", self.backup_path.display())
                        })?;
                    serde_json::from_str::<Vec<ServiceEntry>>(&backup_content).with_context(|| {
                        format!(
                            "failed parsing {} and backup {}",
                            self.path.display(),
                            self.backup_path.display()
                        )
                    })
                } else {
                    Err(primary_error).with_context(|| {
                        format!(
                            "failed parsing {} and backup file does not exist",
                            self.path.display()
                        )
                    })
                }
            }
        }
    }

    async fn ensure_parent_dir(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .await
                .with_context(|| format!("failed creating data directory {}", parent.display()))?;
        }
        Ok(())
    }
}
