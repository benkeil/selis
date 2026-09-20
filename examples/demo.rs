use selis::prelude::*;

/// Rebuilds the household-expenditure sample table from the original
/// Kotlin/Mordant example, using the "classic" (square, double
/// section-separator) theme.
fn build_classic_sample() -> Table {
    Table::build(|t| {
        t.theme(classic());
        t.style(Style::new().fg(Color::from_hex("#4b25b9").unwrap()));
        t.align(Align::Right);
        t.table_borders(Borders::NONE);

        t.header(|h| {
            h.style(Style::new().fg(Color::BrightRed).bold());

            h.row_with(|r| {
                r.cell_borders(Borders::NONE);
                r.cells(["", "", "", ""]);
                r.cell("Percent Change", |c| {
                    c.colspan(2);
                    c.align(Align::Center);
                });
            });

            h.row_with(|r| {
                r.cell_borders(Borders::BOTTOM);
                r.cells(["", "2020", "2021", "2022", "2020-21", "2021-22"]);
            });
        });

        t.body(|b| {
            b.style(Style::new().fg(Color::Green));
            b.cell_borders(Borders::TOP_BOTTOM);

            b.column(0, |c| {
                c.align(Align::Left);
                c.cell_borders(Borders::ALL);
                c.style(Style::new().fg(Color::BrightBlue));
            });
            b.column(4, |c| {
                c.cell_borders(Borders::LEFT_BOTTOM);
                c.style(Style::new().fg(Color::BrightBlue));
            });
            b.column(5, |c| {
                c.style(Style::new().fg(Color::BrightBlue));
            });

            b.row_styles([Style::new(), Style::new().dim()]);

            b.row([
                "Average income before taxes",
                "$84,352",
                "$87,432",
                "$94,003",
                "3.7",
                "7.5",
            ]);
            b.row([
                "Average annual expenditures",
                "$61,332",
                "$66,928",
                "$72,967",
                "9.1",
                "9.0",
            ]);
            b.row(["  Food", "7,310", "8,289", "9,343", "13.4", "12.7"]);
            b.row(["  Housing", "21,417", "22,624", "24,298", "5.6", "7.4"]);
            b.row([
                "  Apparel and services",
                "1,434",
                "1,754",
                "1,945",
                "22.3",
                "10.9",
            ]);
            b.row([
                "  Transportation",
                "9,826",
                "10,961",
                "12,295",
                "11.6",
                "12.2",
            ]);
            b.row(["  Healthcare", "5,177", "5,452", "5,850", "5.3", "7.3"]);
            b.row(["  Entertainment", "2,909", "3,568", "3,458", "22.7", "-3.1"]);
            b.row(["  Education", "1,271", "1,226", "1,335", "-3.5", "8.9"]);
        });

        t.footer(|f| {
            f.style(Style::new().italic());
            f.row_with(|r| {
                r.cells(["Remaining income", "$23,020", "$20,504", "$21,036"]);
            });
        });

        t.caption_bottom(
            Style::new()
                .dim()
                .render("via U.S. Bureau of Labor Statistics"),
        );
    })
}

/// The same data set rendered with the borderless GitHub-style theme.
fn build_github_sample() -> Table {
    Table::build(|t| {
        t.theme(github());
        t.align(Align::Right);

        t.header(|h| {
            h.column(0, |c| {
                c.align(Align::Left);
            });
            h.row(["Header", "2020", "2021", "2022", "2020-21", "2021-22"]);
        });

        t.body(|b| {
            b.column(0, |c| {
                c.align(Align::Left);
            });

            b.row([
                "Average income before taxes",
                "$84,352",
                "$87,432",
                "$94,003",
                "3.7",
                "7.5",
            ]);
            b.row([
                "Average annual expenditures",
                "$61,332",
                "$66,928",
                "$72,967",
                "9.1",
                "9.0",
            ]);
            b.row(["Food", "7,310", "8,289", "9,343", "13.4", "12.7"]);
            b.row(["Housing", "21,417", "22,624", "24,298", "5.6", "7.4"]);
        });

        t.footer(|f| {
            f.row(["Remaining income", "$23,020", "$20,504", "$21,036"]);
        });
    })
}

fn main() {
    println!("classic theme:\n");
    println!("{}", build_classic_sample().render());

    println!("\ngithub theme:\n");
    println!("{}", build_github_sample().render());
}
