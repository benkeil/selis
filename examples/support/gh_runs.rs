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
            id: 35351350164,
            elapsed: "8m52s",
            age: "about 1 day ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da:kotlin-standard-library from 1.4.7 to 1.4.8 …",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 35194005454,
            elapsed: "10m7s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da.libraries:security-overrides-bom from 3.24.7…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 35188565623,
            elapsed: "7m13s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da:kotlin-standard-library from 1.4.1 to 1.4.7 …",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 35187671098,
            elapsed: "7m11s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "feat: remove github repository import as creating new ones fails",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 35104455022,
            elapsed: "6m28s",
            age: "about 3 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da:kotlin-standard-library from 1.3.146 to 1.4.…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 35062452598,
            elapsed: "10m34s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da.libraries:security-overrides-bom from 3.24.5…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 35061487700,
            elapsed: "7m4s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump github/codeql-action from 4.37.9 to 4.38.0 (#659)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 35060883987,
            elapsed: "8m39s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "feat: include `security-overrides-bom` and remove manually added over…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 34963900638,
            elapsed: "10m37s",
            age: "about 4 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da:kotlin-standard-library from 1.3.145 to 1.3.…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 34811410091,
            elapsed: "10m48s",
            age: "about 6 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da:kotlin-standard-library from 1.3.143 to 1.3.…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 34442931693,
            elapsed: "10m39s",
            age: "about 10 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump runs-on/action from 2.2.0 to 2.3.1 (#656)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 34442473059,
            elapsed: "7m53s",
            age: "about 10 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix(DV-10606): use correct Neptune issuer URL on live",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 34112271762,
            elapsed: "8m39s",
            age: "about 13 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "feat(DV-10606): add Neptune API authorizer",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 33866961197,
            elapsed: "19m18s",
            age: "about 15 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump github/codeql-action from 4.37.7 to 4.37.9 (#653)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 33596060805,
            elapsed: "8m53s",
            age: "about 18 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da:kotlin-standard-library from 1.3.142 to 1.3.…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 33475595438,
            elapsed: "10m20s",
            age: "about 19 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump de.otto.pdh.da:kotlin-standard-library from 1.3.140 to 1.3.…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 33362237361,
            elapsed: "10m35s",
            age: "about 20 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump actions/setup-java from 5.6.0 to 6.0.0 (#649)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 33361718943,
            elapsed: "8m39s",
            age: "about 20 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix(DV-10594): force plexus-utils >=4.1.0 (GHSA-6fmv-xxpf-w3cw, CVE-2…",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 33080073370,
            elapsed: "10m53s",
            age: "about 23 days ago",
        },
        WorkflowRun {
            status: Success,
            title: "fix: bump gradle-wrapper from 9.7.0 to 9.7.1 (#648)",
            workflow: "CI & CD",
            branch: "main",
            event: "push",
            id: 33044071621,
            elapsed: "10m27s",
            age: "about 24 days ago",
        },
    ]
}
