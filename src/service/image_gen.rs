use std::{collections::HashMap, fs};

use chrono::NaiveDate;

use crate::{algebra::ImageGenExt, domain::Stats};

pub struct ImageGen {
    template_folder: String,
    output_folder: String,
}

impl ImageGenExt for ImageGen {
    fn generate_overview(&self, stats: &Stats) -> Result<(), anyhow::Error> {
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

    fn generate_languages(&self, stats: &Stats) -> Result<(), anyhow::Error> {
        let svg_content = fs::read_to_string(format!("{}/languages.svg", self.template_folder))?;
        let mut progress = "".to_string();
        let mut lang_list = "".to_string();
        let mut tags_map = HashMap::new();

        for (idx, (language, data)) in stats.languages().iter().enumerate() {
            let proportion = format!("{:.2}", data.proportion());
            let progress_tmp = format!(
                r#"<span style="background-color: {}; width: {}%;" class="progress-item"></span>"#,
                data.color(),
                &proportion
            );

            let lang_list_tmp = format!(
                r#"<li style="animation-delay: {}ms;">
<svg xmlns="http://www.w3.org/2000/svg" class="octicon" style="fill:{};"
viewBox="0 0 16 16" version="1.1" width="16" height="16"><path
fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
<span class="lang">{}</span>
<span class="percent">{}%</span>
</li>

            "#,
                150 * idx,
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

    fn generate_contributions_grid(&self, stats: &Stats) -> Result<(), anyhow::Error> {
        let svg_content =
            fs::read_to_string(format!("{}/contribution_grid.svg", self.template_folder))?;
        let mut grid: String = "".to_string();
        let mut months: Vec<(String, usize)> = Vec::new();

        let mut animation_delay = 0;

        for (week_index, week) in stats.contribution_calendar().iter().enumerate() {
            if let Some(first_day) = week.contribution_days.last() {
                let date = first_day.date.to_string();
                let naive_date = NaiveDate::parse_from_str(&date, " %Y-%m-%d")?;
                // Get the first 3 letters of the month
                let month = naive_date.format("%b").to_string();
                if months
                    .last()
                    .is_none_or(|(last_month, _)| *last_month != month)
                {
                    months.push((month, week_index));
                }
            }

            grid.push_str("<div>");
            for day in week.contribution_days.iter() {
                let color = day.color.clone();

                grid.push_str(&format!(
                    r#"<div class="contribution_cell" style="background-color: {color}; animation-delay: {animation_delay}ms;"></div>"#,
                ));

                animation_delay += 10; // Increment delay for the next cell
            }
            grid.push_str("</div>");
        }

        let month_labels = months
            .iter()
              .fold(("".to_string(), 150), |(mut month_labels, animation_delay), (month, week_index)| {
             let x = 40 + week_index * 12; // Adjust the x position based on the week index
             month_labels.push_str(&format!(
                r#"<text style="animation-delay: {animation_delay}ms" x="{x}" y="40" class="month-label">{month}</text>"#
            ));
            (month_labels, animation_delay + 150)
        }).0;

        let modified_content = Self::replace_tags(
            svg_content,
            &HashMap::from([
                ("grid".to_string(), grid),
                ("months".to_string(), month_labels),
            ]),
        )?;

        fs::write(
            format!("{}/contribution_grid.svg", self.output_folder),
            modified_content,
        )?;

        Ok(())
    }
}

impl ImageGen {
    pub fn new(template_folder: String, output_folder: String) -> Self {
        Self {
            template_folder,
            output_folder,
        }
    }

    fn replace_tags(
        content: String,
        replacements: &HashMap<String, String>,
    ) -> Result<String, anyhow::Error> {
        let mut modified_content = content;
        for (tag, value) in replacements {
            let pattern = format!(r"{{{{ {tag} }}}}");

            modified_content = modified_content.replace(&pattern, value);
        }

        Ok(modified_content)
    }
}
