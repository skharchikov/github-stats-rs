use std::{collections::HashMap, fs};

use crate::domain::Stats;

pub struct ImageGen {
    template_folder: String,
    output_folder: String,
}

impl ImageGen {
    pub fn new(template_folder: String, output_folder: String) -> Self {
        Self {
            template_folder,
            output_folder,
        }
    }

    pub fn generate_overview(&self, stats: &Stats) -> Result<(), anyhow::Error> {
        let svg_content = fs::read_to_string(format!("{}/overview.svg", self.template_folder))?;
        let mut tags_map = HashMap::new();

        tags_map.insert("name".to_string(), stats.name().to_string());
        tags_map.insert("stars".to_string(), stats.stargazers().to_string());
        tags_map.insert("forks".to_string(), stats.forks().to_string());
        tags_map.insert(
            "contributions".to_string(),
            stats.total_contributions().to_string(),
        );
        let (added, removed) = stats.lines_changed();
        tags_map.insert("lines_changed".to_string(), format!("{}", added + removed));
        tags_map.insert("views".to_string(), stats.views().to_string());
        tags_map.insert("repos".to_string(), stats.repos().iter().len().to_string());

        fs::create_dir_all(&self.output_folder)?;
        let modified_content = Self::replace_tags(svg_content, &tags_map)?;

        fs::write(
            format!("{}/overview.svg", self.output_folder),
            modified_content,
        )?;

        Ok(())
    }

    pub fn generate_languages(&self, stats: &Stats) -> Result<(), anyhow::Error> {
        let svg_content = fs::read_to_string(format!("{}/languages.svg", self.template_folder))?;
        let mut progress = "".to_string();
        let mut lang_list = "".to_string();
        let mut tags_map = HashMap::new();

        let mut sorted_languages = stats.languages().iter().collect::<Vec<_>>();
        sorted_languages.sort_by(|a, b| a.1.size().cmp(&b.1.size()));

        for (language, data) in sorted_languages.iter().rev() {
            let proportion = format!("{:.2}", data.proportion());
            let progress_tmp = format!(
                r#"<span style="background-color: {}; width: {}%;" class="progress-item"></span>"#,
                data.color(),
                &proportion
            );

            let lang_list_tmp = format!(
                r#"<li style="animation-delay: 150ms;">
<svg xmlns="http://www.w3.org/2000/svg" class="octicon" style="fill:{};"
viewBox="0 0 16 16" version="1.1" width="16" height="16"><path
fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
<span class="lang">{}</span>
<span class="percent">{}%</span>
</li>

            "#,
                data.color(),
                language,
                &proportion
            );
            progress.push_str(&progress_tmp);
            lang_list.push_str(&lang_list_tmp);
        }

        tags_map.insert("progress".to_string(), progress);
        tags_map.insert("lang_list".to_string(), lang_list);

        fs::create_dir_all(&self.output_folder)?;
        let modified_content = Self::replace_tags(svg_content, &tags_map)?;

        fs::write(
            format!("{}/languages.svg", self.output_folder),
            modified_content,
        )?;

        Ok(())
    }

    fn replace_tags(
        content: String,
        replacements: &HashMap<String, String>,
    ) -> Result<String, anyhow::Error> {
        let mut modified_content = content;
        // Use regex to match each tag in the replacements HashMap
        for (tag, value) in replacements {
            // Create a regex to match the tag
            let pattern = format!(r"{{{{ {} }}}}", tag);
            // let re = Regex::new(&pattern)?;

            modified_content = modified_content.replace(&pattern, value);

            // modified_content = re.replace(&modified_content, value.as_str()).to_string();
        }

        Ok(modified_content)
    }
}
