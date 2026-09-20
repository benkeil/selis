//! Renders the `gh run list` sample dataset (see `examples/support/gh_runs.rs`)
//! as a table, to prove the shared dataset compiles and works with the DSL.
//! Follow-up examples can reuse `support::gh_runs::sample_runs()` to render
//! the very same data with different styles/themes.

#[path = "support/mod.rs"]
mod support;

use selis::prelude::*;
use support::gh_runs::sample_runs;

fn main() {
    let runs = sample_runs();

    let table = Table::build(|t| {
        t.theme(minimal_zebra());
        t.show_header(true);
        t.show_footer(true);

        t.header(|h| {
            h.row(|r| {
                r.cell("status");
                r.cell("title");
                r.cell("workflow");
                r.cell("branch");
                r.cell("event");
                r.cell("id");
                r.cell("elapsed");
                r.cell("age");
            });
        });

        t.body(|b| {
            for run in &runs {
                b.row(|r| {
                    r.cell(run.status.glyph().to_string())
                        .fg(run.status.color())
                        .bold();
                    r.cell(run.title.to_string());
                    r.cell(run.workflow.to_string());
                    r.cell(run.branch.to_string());
                    r.cell(run.event.to_string());
                    r.cell(run.id.to_string()).fg(Color::Cyan);
                    r.cell(run.elapsed.to_string());
                    r.cell(run.age.to_string());
                });
            }
        });
    });

    println!("{}", table.render());
}
