use std::{fmt, io, path::PathBuf, process::Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateKind {
    Stopped,
    Starting,
    Running,
    Failed,
}

impl fmt::Display for StateKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Stopped => "stopped",
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Failed => "failed",
        };
        formatter.write_str(label)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ServiceState {
    Stopped { last_exit_code: Option<i32> },
    Starting,
    Running { pid: u32 },
    Failed { message: String },
}

impl ServiceState {
    pub fn kind(&self) -> StateKind {
        match self {
            Self::Stopped { .. } => StateKind::Stopped,
            Self::Starting => StateKind::Starting,
            Self::Running { .. } => StateKind::Running,
            Self::Failed { .. } => StateKind::Failed,
        }
    }

    pub fn apply(&mut self, event: ServiceEvent) -> Result<(), TransitionError> {
        let from = self.kind();
        let next = match (&*self, event) {
            (Self::Stopped { .. }, ServiceEvent::StartRequested) => Self::Starting,
            (Self::Starting, ServiceEvent::ProcessSpawned { pid }) => Self::Running { pid },
            (Self::Starting, ServiceEvent::LaunchFailed { message }) => Self::Failed { message },
            (Self::Running { .. }, ServiceEvent::ProcessExited { code }) => Self::Stopped {
                last_exit_code: code,
            },
            (Self::Running { .. }, ServiceEvent::WaitFailed { message }) => {
                Self::Failed { message }
            }
            (_, attempted) => {
                return Err(TransitionError {
                    from,
                    event: attempted.kind(),
                });
            }
        };
        *self = next;
        Ok(())
    }
}

impl Default for ServiceState {
    fn default() -> Self {
        Self::Stopped {
            last_exit_code: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ServiceEvent {
    StartRequested,
    ProcessSpawned { pid: u32 },
    ProcessExited { code: Option<i32> },
    LaunchFailed { message: String },
    WaitFailed { message: String },
}

impl ServiceEvent {
    fn kind(&self) -> EventKind {
        match self {
            Self::StartRequested => EventKind::StartRequested,
            Self::ProcessSpawned { .. } => EventKind::ProcessSpawned,
            Self::ProcessExited { .. } => EventKind::ProcessExited,
            Self::LaunchFailed { .. } => EventKind::LaunchFailed,
            Self::WaitFailed { .. } => EventKind::WaitFailed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    StartRequested,
    ProcessSpawned,
    ProcessExited,
    LaunchFailed,
    WaitFailed,
}

impl fmt::Display for EventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::StartRequested => "start requested",
            Self::ProcessSpawned => "process spawned",
            Self::ProcessExited => "process exited",
            Self::LaunchFailed => "launch failed",
            Self::WaitFailed => "wait failed",
        };
        formatter.write_str(label)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransitionError {
    pub from: StateKind,
    pub event: EventKind,
}

impl fmt::Display for TransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot apply {} while service is {}",
            self.event, self.from
        )
    }
}

impl std::error::Error for TransitionError {}

#[derive(Debug, PartialEq, Eq)]
pub struct ServiceDefinition {
    name: String,
    program: PathBuf,
    args: Vec<String>,
}

impl ServiceDefinition {
    pub fn new(name: impl Into<String>, program: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            program: program.into(),
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, value: impl Into<String>) -> Self {
        self.args.push(value.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RunSummary {
    pub service_name: String,
    pub pid: u32,
    pub exit_code: Option<i32>,
}

#[derive(Debug)]
pub enum RunError {
    InvalidTransition(TransitionError),
    Launch { service: String, source: io::Error },
    Wait { service: String, source: io::Error },
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition(error) => error.fmt(formatter),
            Self::Launch { service, source } => {
                write!(formatter, "could not launch service {service}: {source}")
            }
            Self::Wait { service, source } => {
                write!(formatter, "could not wait for service {service}: {source}")
            }
        }
    }
}

impl std::error::Error for RunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTransition(error) => Some(error),
            Self::Launch { source, .. } | Self::Wait { source, .. } => Some(source),
        }
    }
}

impl From<TransitionError> for RunError {
    fn from(error: TransitionError) -> Self {
        Self::InvalidTransition(error)
    }
}

#[derive(Debug)]
pub struct Supervisor {
    definition: ServiceDefinition,
    state: ServiceState,
}

impl Supervisor {
    pub fn new(definition: ServiceDefinition) -> Self {
        Self {
            definition,
            state: ServiceState::default(),
        }
    }

    pub fn state(&self) -> &ServiceState {
        &self.state
    }

    pub fn run_to_completion(&mut self) -> Result<RunSummary, RunError> {
        self.state.apply(ServiceEvent::StartRequested)?;

        let mut child = match Command::new(&self.definition.program)
            .args(&self.definition.args)
            .spawn()
        {
            Ok(child) => child,
            Err(source) => {
                let message = source.to_string();
                self.state.apply(ServiceEvent::LaunchFailed { message })?;
                return Err(RunError::Launch {
                    service: self.definition.name.clone(),
                    source,
                });
            }
        };

        let pid = child.id();
        self.state.apply(ServiceEvent::ProcessSpawned { pid })?;

        let status = match child.wait() {
            Ok(status) => status,
            Err(source) => {
                let message = source.to_string();
                self.state.apply(ServiceEvent::WaitFailed { message })?;
                return Err(RunError::Wait {
                    service: self.definition.name.clone(),
                    source,
                });
            }
        };
        let exit_code = status.code();
        self.state
            .apply(ServiceEvent::ProcessExited { code: exit_code })?;

        Ok(RunSummary {
            service_name: self.definition.name.clone(),
            pid,
            exit_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_successful_service_lifecycle() {
        let mut state = ServiceState::default();
        state.apply(ServiceEvent::StartRequested).unwrap();
        assert_eq!(state, ServiceState::Starting);

        state
            .apply(ServiceEvent::ProcessSpawned { pid: 4132 })
            .unwrap();
        assert_eq!(state, ServiceState::Running { pid: 4132 });

        state
            .apply(ServiceEvent::ProcessExited { code: Some(0) })
            .unwrap();
        assert_eq!(
            state,
            ServiceState::Stopped {
                last_exit_code: Some(0)
            }
        );
    }

    #[test]
    fn rejects_an_exit_before_a_process_is_running() {
        let mut state = ServiceState::default();
        let error = state
            .apply(ServiceEvent::ProcessExited { code: Some(0) })
            .unwrap_err();

        assert_eq!(
            error,
            TransitionError {
                from: StateKind::Stopped,
                event: EventKind::ProcessExited,
            }
        );
        assert_eq!(state, ServiceState::default());
    }

    #[test]
    fn records_a_launch_failure() {
        let mut state = ServiceState::default();
        state.apply(ServiceEvent::StartRequested).unwrap();
        state
            .apply(ServiceEvent::LaunchFailed {
                message: "program not found".to_owned(),
            })
            .unwrap();

        assert_eq!(
            state,
            ServiceState::Failed {
                message: "program not found".to_owned()
            }
        );
    }

    #[test]
    fn constructs_a_named_service_without_cloning_inputs() {
        let definition = ServiceDefinition::new("atlas-worker", "worker-bin").arg("--once");
        assert_eq!(definition.name(), "atlas-worker");
        assert_eq!(definition.program, PathBuf::from("worker-bin"));
        assert_eq!(definition.args, vec!["--once"]);
    }

    #[test]
    fn launches_a_process_and_records_its_exit_code() {
        let definition = successful_test_service();
        let mut supervisor = Supervisor::new(definition);
        let summary = supervisor.run_to_completion().unwrap();

        assert_eq!(summary.service_name, "test-worker");
        assert!(summary.pid > 0);
        assert_eq!(summary.exit_code, Some(7));
        assert_eq!(
            supervisor.state(),
            &ServiceState::Stopped {
                last_exit_code: Some(7)
            }
        );
    }

    #[cfg(windows)]
    fn successful_test_service() -> ServiceDefinition {
        ServiceDefinition::new("test-worker", "cmd")
            .arg("/C")
            .arg("exit 7")
    }

    #[cfg(not(windows))]
    fn successful_test_service() -> ServiceDefinition {
        ServiceDefinition::new("test-worker", "sh")
            .arg("-c")
            .arg("exit 7")
    }
}
