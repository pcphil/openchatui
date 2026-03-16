use super::{
    ChangeProposal, SandboxConfig, SandboxError, SandboxEvent, SandboxInfo, SandboxRuntime,
    SandboxStatus,
};
use async_trait::async_trait;
use bollard::container::{
    Config, CreateContainerOptions, ListContainersOptions, LogOutput, RemoveContainerOptions,
    StartContainerOptions, StopContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::HostConfig;
use bollard::Docker;
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

const SANDBOX_IMAGE: &str = "openchatui/sandbox:latest";
const SANDBOX_LABEL: &str = "openchatui.sandbox";

pub struct DockerRuntime {
    docker: Arc<Docker>,
}

impl DockerRuntime {
    pub fn new() -> Result<Self, SandboxError> {
        let docker =
            Docker::connect_with_local_defaults().map_err(|e| SandboxError::Docker(e.to_string()))?;
        Ok(Self {
            docker: Arc::new(docker),
        })
    }

    fn container_name(sandbox_id: &str) -> String {
        format!("openchatui-sandbox-{}", sandbox_id)
    }

    async fn find_container_id(&self, sandbox_id: &str) -> Result<String, SandboxError> {
        let name = Self::container_name(sandbox_id);
        let mut filters = HashMap::new();
        filters.insert("name".to_string(), vec![name.clone()]);
        filters.insert("label".to_string(), vec![SANDBOX_LABEL.to_string()]);

        let containers = self
            .docker
            .list_containers(Some(ListContainersOptions {
                all: true,
                filters,
                ..Default::default()
            }))
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        containers
            .first()
            .and_then(|c| c.id.clone())
            .ok_or_else(|| SandboxError::NotFound(sandbox_id.to_string()))
    }

    fn build_host_config(config: &SandboxConfig) -> HostConfig {
        let memory = config
            .memory_limit
            .as_ref()
            .and_then(|m| parse_memory_limit(m));

        let nano_cpus = config.cpu_limit.map(|c| (c * 1_000_000_000.0) as i64);

        let network_mode = if config.network_enabled.unwrap_or(false) {
            None
        } else {
            Some("none".to_string())
        };

        let mut binds = Vec::new();
        if let Some(ref dir) = config.project_dir {
            binds.push(format!("{}:/workspace", dir));
        }

        HostConfig {
            memory,
            nano_cpus,
            network_mode,
            binds: if binds.is_empty() { None } else { Some(binds) },
            security_opt: Some(vec!["no-new-privileges".to_string()]),
            readonly_rootfs: Some(true),
            tmpfs: Some(HashMap::from([
                ("/tmp".to_string(), "rw,noexec,nosuid,size=256m".to_string()),
                ("/run".to_string(), "rw,noexec,nosuid,size=64m".to_string()),
                ("/home/sandbox".to_string(), "rw,noexec,nosuid,size=512m".to_string()),
            ])),
            ..Default::default()
        }
    }
}

fn parse_memory_limit(s: &str) -> Option<i64> {
    let s = s.trim().to_lowercase();
    if let Some(num) = s.strip_suffix('g') {
        num.parse::<i64>().ok().map(|n| n * 1024 * 1024 * 1024)
    } else if let Some(num) = s.strip_suffix('m') {
        num.parse::<i64>().ok().map(|n| n * 1024 * 1024)
    } else {
        s.parse::<i64>().ok()
    }
}

#[async_trait]
impl SandboxRuntime for DockerRuntime {
    async fn create(
        &self,
        conversation_id: &str,
        config: SandboxConfig,
    ) -> Result<SandboxInfo, SandboxError> {
        let sandbox_id = uuid::Uuid::new_v4().to_string();
        let container_name = Self::container_name(&sandbox_id);

        let mut labels = HashMap::new();
        labels.insert(SANDBOX_LABEL.to_string(), "true".to_string());
        labels.insert(
            "openchatui.conversation_id".to_string(),
            conversation_id.to_string(),
        );
        labels.insert("openchatui.sandbox_id".to_string(), sandbox_id.clone());

        let mut env = vec![
            format!("SANDBOX_ID={}", sandbox_id),
            format!("CONVERSATION_ID={}", conversation_id),
        ];
        if let Some(ref extra_env) = config.environment {
            for (k, v) in extra_env {
                env.push(format!("{}={}", k, v));
            }
        }

        let host_config = Self::build_host_config(&config);

        let container_config = Config {
            image: Some(SANDBOX_IMAGE.to_string()),
            hostname: Some("sandbox".to_string()),
            working_dir: Some("/workspace".to_string()),
            labels: Some(labels),
            env: Some(env),
            host_config: Some(host_config),
            tty: Some(true),
            open_stdin: Some(true),
            cmd: Some(vec!["sleep".to_string(), "infinity".to_string()]),
            ..Default::default()
        };

        self.docker
            .create_container(
                Some(CreateContainerOptions {
                    name: container_name.as_str(),
                    platform: None,
                }),
                container_config,
            )
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        Ok(SandboxInfo {
            sandbox_id,
            conversation_id: conversation_id.to_string(),
            status: SandboxStatus::Creating,
            container_id: Some(container_name),
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn start(&self, sandbox_id: &str) -> Result<(), SandboxError> {
        let name = Self::container_name(sandbox_id);
        self.docker
            .start_container(&name, None::<StartContainerOptions<String>>)
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;
        Ok(())
    }

    async fn exec(
        &self,
        sandbox_id: &str,
        command: Vec<String>,
        event_tx: mpsc::Sender<SandboxEvent>,
    ) -> Result<i64, SandboxError> {
        let name = Self::container_name(sandbox_id);

        let exec = self
            .docker
            .create_exec(
                &name,
                CreateExecOptions {
                    cmd: Some(command),
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    attach_stdin: Some(true),
                    tty: Some(false),
                    working_dir: Some("/workspace".to_string()),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        let start_result = self
            .docker
            .start_exec(&exec.id, None)
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        match start_result {
            StartExecResults::Attached { mut output, .. } => {
                let mut proposal_buffer = String::new();
                let mut in_proposal = false;

                while let Some(msg) = output.next().await {
                    match msg {
                        Ok(log) => {
                            let (stream, text) = match &log {
                                LogOutput::StdOut { message } => {
                                    ("stdout".to_string(), String::from_utf8_lossy(message).to_string())
                                }
                                LogOutput::StdErr { message } => {
                                    ("stderr".to_string(), String::from_utf8_lossy(message).to_string())
                                }
                                _ => continue,
                            };

                            // Check for proposal delimiters
                            if text.contains("---PROPOSAL_START---") {
                                in_proposal = true;
                                proposal_buffer.clear();
                                continue;
                            }
                            if text.contains("---PROPOSAL_END---") {
                                in_proposal = false;
                                if let Ok(proposal) =
                                    serde_json::from_str::<ChangeProposal>(&proposal_buffer)
                                {
                                    let _ = event_tx
                                        .send(SandboxEvent::ProposalReady(proposal))
                                        .await;
                                }
                                proposal_buffer.clear();
                                continue;
                            }

                            if in_proposal {
                                proposal_buffer.push_str(&text);
                            } else {
                                let _ = event_tx
                                    .send(SandboxEvent::Output {
                                        stream,
                                        text,
                                    })
                                    .await;
                            }
                        }
                        Err(e) => {
                            let _ = event_tx
                                .send(SandboxEvent::Error(e.to_string()))
                                .await;
                        }
                    }
                }
            }
            StartExecResults::Detached => {}
        }

        // Get exit code
        let inspect = self
            .docker
            .inspect_exec(&exec.id)
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        Ok(inspect.exit_code.unwrap_or(-1))
    }

    async fn stop(&self, sandbox_id: &str) -> Result<(), SandboxError> {
        let name = Self::container_name(sandbox_id);
        self.docker
            .stop_container(
                &name,
                Some(StopContainerOptions { t: 10 }),
            )
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;
        Ok(())
    }

    async fn destroy(&self, sandbox_id: &str) -> Result<(), SandboxError> {
        let name = Self::container_name(sandbox_id);
        self.docker
            .remove_container(
                &name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;
        Ok(())
    }

    async fn info(&self, sandbox_id: &str) -> Result<SandboxInfo, SandboxError> {
        let name = Self::container_name(sandbox_id);
        let inspect = self
            .docker
            .inspect_container(&name, None)
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        let status = match inspect.state.as_ref().and_then(|s| s.status) {
            Some(bollard::models::ContainerStateStatusEnum::RUNNING) => SandboxStatus::Running,
            Some(bollard::models::ContainerStateStatusEnum::CREATED) => SandboxStatus::Creating,
            Some(bollard::models::ContainerStateStatusEnum::EXITED) => SandboxStatus::Stopped,
            _ => SandboxStatus::Failed,
        };

        let conversation_id = inspect
            .config
            .as_ref()
            .and_then(|c| c.labels.as_ref())
            .and_then(|l| l.get("openchatui.conversation_id"))
            .cloned()
            .unwrap_or_default();

        let created_at = inspect.created.unwrap_or_default();

        Ok(SandboxInfo {
            sandbox_id: sandbox_id.to_string(),
            conversation_id,
            status,
            container_id: inspect.id,
            created_at,
        })
    }

    async fn list(&self) -> Result<Vec<SandboxInfo>, SandboxError> {
        let mut filters = HashMap::new();
        filters.insert("label".to_string(), vec![SANDBOX_LABEL.to_string()]);

        let containers = self
            .docker
            .list_containers(Some(ListContainersOptions {
                all: true,
                filters,
                ..Default::default()
            }))
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        let mut results = Vec::new();
        for c in containers {
            let sandbox_id = c
                .labels
                .as_ref()
                .and_then(|l| l.get("openchatui.sandbox_id"))
                .cloned()
                .unwrap_or_default();
            let conversation_id = c
                .labels
                .as_ref()
                .and_then(|l| l.get("openchatui.conversation_id"))
                .cloned()
                .unwrap_or_default();

            let status = match c.state.as_deref() {
                Some("running") => SandboxStatus::Running,
                Some("created") => SandboxStatus::Creating,
                Some("exited") => SandboxStatus::Stopped,
                _ => SandboxStatus::Failed,
            };

            results.push(SandboxInfo {
                sandbox_id,
                conversation_id,
                status,
                container_id: c.id,
                created_at: c.created.map(|t| t.to_string()).unwrap_or_default(),
            });
        }

        Ok(results)
    }

    async fn write_file(
        &self,
        sandbox_id: &str,
        path: &str,
        content: &[u8],
    ) -> Result<(), SandboxError> {
        let name = Self::container_name(sandbox_id);

        // Build a tar archive containing the file
        let mut tar_buf = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut tar_buf);
            let mut header = tar::Header::new_gnu();
            header.set_size(content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();

            let file_name = std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file");

            builder
                .append_data(&mut header, file_name, content)
                .map_err(|e| SandboxError::Io(e))?;
            builder.finish().map_err(|e| SandboxError::Io(e))?;
        }

        let parent_dir = std::path::Path::new(path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("/workspace");

        self.docker
            .upload_to_container(
                &name,
                Some(bollard::container::UploadToContainerOptions {
                    path: parent_dir.to_string(),
                    ..Default::default()
                }),
                tar_buf.into(),
            )
            .await
            .map_err(|e| SandboxError::Docker(e.to_string()))?;

        Ok(())
    }

    async fn read_file(&self, sandbox_id: &str, path: &str) -> Result<Vec<u8>, SandboxError> {
        let name = Self::container_name(sandbox_id);

        let stream = self
            .docker
            .download_from_container(
                &name,
                Some(bollard::container::DownloadFromContainerOptions { path: path.to_string() }),
            );

        let mut bytes = Vec::new();
        futures::pin_mut!(stream);
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| SandboxError::Docker(e.to_string()))?;
            bytes.extend_from_slice(&chunk);
        }

        // The response is a tar archive; extract the file content
        let mut archive = tar::Archive::new(bytes.as_slice());
        for entry in archive.entries().map_err(|e| SandboxError::Io(e))? {
            let mut entry = entry.map_err(|e| SandboxError::Io(e))?;
            let mut content = Vec::new();
            std::io::Read::read_to_end(&mut entry, &mut content)
                .map_err(|e| SandboxError::Io(e))?;
            return Ok(content);
        }

        Err(SandboxError::NotFound(format!("File not found: {}", path)))
    }
}
