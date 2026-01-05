pub struct DisplayLines;

impl DisplayLines {
    pub fn indent(s: String) -> String {
        s.lines()
            .map(|l| format!("\t{}", l))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn indent_n(s: String, n: usize) -> String {
        let indent = "\t".repeat(n);
        s.lines()
            .map(|l| format!("{}{}", indent, l))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
