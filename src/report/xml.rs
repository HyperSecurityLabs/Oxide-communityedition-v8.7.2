// ----------------------------------------------------------------------------
//  xml.rs — XML report generator
// ----------------------------------------------------------------------------
//  XML report generator — exports findings in XML format for interoperability.
//
//  --- Developers ---------------------------------------------------------------
//  khaninkali             — разработчик / core engineer (Rust backend, logic)
//  Lyara Koroleva         — дизайнер / blazing fast CLI & interface design
//  HsecDevelopers         — 测试 / テスト / testing & QA (integration, validation)
//  projectk 2091         — HyperSecurityOffensiveLabs lineage
// ----------------------------------------------------------------------------
//
//
// ---------------------------------------------------------------------------
//   LICENSE / ライセンス — GNU General Public License v3 (GPL-3.0)
// ---------------------------------------------------------------------------
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  OXIDE v8.7.2-community-edition — HyperSecurityOffensiveLabs
// ---------------------------------------------------------------------------
//
//

//  XmlReport: XMLレポート生成器
//  Exports findings as XML for interoperability with external security tools.
//  スキーマ準拠:
//   <scan xmlns="http://oxide.org/schema">
//     <metadata> — tool name + version
//     <findings> — <finding>要素のリスト (url/severity/title/description/evidence/remediation)
//  エスケープ: &lt; &gt; &amp; &quot; &apos; をXMLエンティティに変換
//  escape_xml: 5つのXML特殊文字を安全にエスケープ
pub struct XmlReport;

impl XmlReport {
    //  generate_header: XMLドキュメントヘッダー
    //  Generates XML declaration + root <scan> element with metadata.
    pub fn generate_header() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<scan xmlns="http://oxide.org/schema">
    <metadata>
        <tool>OXIDE</tool>
        <version>1.0.0</version>
    </metadata>
    <findings>
"#.to_string()
    }

    //  generate_finding: XML発見要素生成
    //  Generates a single <finding> element with all fields XML-escaped.
    pub fn generate_finding(
        url: &str,
        severity: &str,
        title: &str,
        description: &str,
        evidence: &str,
        remediation: &str,
    ) -> String {
        format!(
        r#"        <finding>
            <url>{}</url>
            <severity>{}</severity>
            <title>{}</title>
            <description>{}</description>
            <evidence>{}</evidence>
            <remediation>{}</remediation>
        </finding>
"#,
            Self::escape_xml(url),
            Self::escape_xml(severity),
            Self::escape_xml(title),
            Self::escape_xml(description),
            Self::escape_xml(evidence),
            Self::escape_xml(remediation)
        )
    }

    //  generate_footer: XMLフッター
    //  Closes <findings> and <scan> elements.
    pub fn generate_footer() -> String {
        r#"    </findings>
</scan>"#.to_string()
    }

    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}
