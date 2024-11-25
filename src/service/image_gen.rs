use std::{collections::HashMap, fs};

use regex::Regex;

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

        todo!()
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
