//! Sample data captured from `gh run list --workflow cicd.yaml`, used by the
//! table examples to demonstrate rendering the same real-world dataset with
//! different styles/themes.

/// The status glyph `gh run list` prints in its first column. Only
/// `Success` appears in the captured sample data, but `Failure`/`InProgress`
/// are included since real `gh run list` output can contain them too.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Success,
    Failure,
    InProgress,
}

impl RunStatus {
    /// The glyph `gh` itself prints for this status (`✓`, `X`, `•`, ...).
    pub fn glyph(self) -> &'static str {
        match self {
            RunStatus::Success => "✓",
            RunStatus::Failure => "X",
            RunStatus::InProgress => "•",
        }
    }

    /// The color a status should be rendered in: green/orange/red.
    pub fn color(self) -> selis::Color {
        match self {
            RunStatus::Success => selis::Color::Green,
            RunStatus::Failure => selis::Color::Red,
            RunStatus::InProgress => selis::Color::Yellow,
        }
    }
}

/// One row of `gh run list` output.
#[derive(Debug, Clone)]
pub struct WorkflowRun {
    pub status: RunStatus,
    pub title: &'static str,
    pub workflow: &'static str,
    pub branch: &'static str,
    pub event: &'static str,
    pub id: u64,
    pub elapsed: &'static str,
    pub age: &'static str,
}

/// Returns the sample dataset captured from `gh run list --workflow
/// cicd.yaml`, so different examples can render the same data with
/// different table styles.
pub fn sample_runs() -> Vec<WorkflowRun> {
    use RunStatus::Success;

    vec![
        WorkflowRun {
            status: Success,
            title: "feat: allow specifying legend position for CloudWatch metrics widgets",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45123456789,
            elapsed: "8m52s",
            age: "about 1 day ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app:kotlin-standard-library from 1.4.7 to 1.4.8 (#661)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45119876543,
            elapsed: "10m7s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app.libraries:security-overrides-bom from 3.24.7 to 3.24.8 (#658)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45118765432,
            elapsed: "7m13s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app:kotlin-standard-library from 1.4.1 to 1.4.7 (#657)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45117654321,
            elapsed: "7m11s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "feat: remove github repository import as creating new ones fails",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45109876543,
            elapsed: "6m28s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app:kotlin-standard-library from 1.3.146 to 1.4.0 (#655)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45098765432,
            elapsed: "10m34s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app.libraries:security-overrides-bom from 3.24.5 to 3.24.6 (#654)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45097654321,
            elapsed: "7m4s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump github/codeql-action from 4.37.9 to 4.38.0 (#659)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 45096543210,
            elapsed: "8m39s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "feat: include `security-overrides-bom` and remove manually added overrides that duplicated it",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44987654321,
            elapsed: "10m37s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app:kotlin-standard-library from 1.3.145 to 1.3.146 (#652)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44876543210,
            elapsed: "10m48s",
            age: "about 6 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app:kotlin-standard-library from 1.3.143 to 1.3.145 (#650)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44765432109,
            elapsed: "10m39s",
            age: "about 10 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump runs-on/action from 2.2.0 to 2.3.1 (#656)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44754321098,
            elapsed: "7m53s",
            age: "about 10 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix(PROJ-4821): use correct Neptune issuer URL on live",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44654321098,
            elapsed: "8m39s",
            age: "about 13 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "feat(PROJ-4821): add Neptune API authorizer",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44543210987,
            elapsed: "19m18s",
            age: "about 15 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump github/codeql-action from 4.37.7 to 4.37.9 (#653)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44432109876,
            elapsed: "8m53s",
            age: "about 18 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app:kotlin-standard-library from 1.3.142 to 1.3.143 (#651)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44321098765,
            elapsed: "10m20s",
            age: "about 19 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump com.example.app:kotlin-standard-library from 1.3.140 to 1.3.142 (#649)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44219876543,
            elapsed: "10m35s",
            age: "about 20 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump actions/setup-java from 5.6.0 to 6.0.0 (#649)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44109876543,
            elapsed: "8m39s",
            age: "about 20 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix(PROJ-4802): force plexus-utils >=4.1.0 (GHSA-6fmv-xxpf-w3cw, CVE-2023-2976)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44098765432,
            elapsed: "10m53s",
            age: "about 23 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump gradle-wrapper from 9.7.0 to 9.7.1 (#648)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 44087654321,
            elapsed: "10m27s",
            age: "about 24 days ago",
        },
    ]
}
