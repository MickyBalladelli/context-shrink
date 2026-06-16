use crate::budget::ProcessedFile;

pub fn format_repository_context(files: &[ProcessedFile]) -> String {
    let mut output = String::new();
    output.push_str("<repository_context>\n");

    for file in files {
        output.push_str("<file path=\"");
        push_xml_escaped(&mut output, &file.path);
        output.push_str("\" level=\"");
        output.push_str(&file.level.as_u8().to_string());
        output.push_str("\">");
        push_xml_escaped(&mut output, file.content());
        output.push_str("</file>\n");
    }

    output.push_str("</repository_context>\n");
    output
}

fn push_xml_escaped(output: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            _ => output.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::ProcessedFile;
    use crate::parser::{CompressionLevel, FileVariants};

    #[test]
    fn escapes_xml_in_paths_and_content() {
        let files = vec![ProcessedFile::new(
            "src/<bad>&\"name\".rs".to_owned(),
            CompressionLevel::Skeleton,
            FileVariants {
                full: Some("ignored".to_owned()),
                skeleton: "if a < b && name == 'x' { \"yes\" }".to_owned(),
                tree_map: String::new(),
            },
        )];

        let xml = format_repository_context(&files);

        assert!(xml.contains("path=\"src/&lt;bad&gt;&amp;&quot;name&quot;.rs\""));
        assert!(xml.contains("a &lt; b &amp;&amp; name == &apos;x&apos;"));
        assert!(xml.contains("{ &quot;yes&quot; }"));
    }
}
