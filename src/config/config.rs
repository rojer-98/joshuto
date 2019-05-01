use serde_derive::Deserialize;

use super::{parse_config_file, ConfigStructure, Flattenable};
use crate::sort;

use crate::CONFIG_FILE;

#[derive(Clone, Debug, Deserialize)]
pub struct SortRawOption {
    pub show_hidden: Option<bool>,
    pub directories_first: Option<bool>,
    pub case_sensitive: Option<bool>,
    pub reverse: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoshutoRawConfig {
    scroll_offset: Option<usize>,
    tilde_in_titlebar: Option<bool>,
    show_preview: Option<bool>,
    max_preview_size: Option<u64>,
    sort_method: Option<String>,
    sort_option: Option<SortRawOption>,
    column_ratio: Option<[usize; 3]>,
}

impl Flattenable<JoshutoConfig> for JoshutoRawConfig {
    fn flatten(self) -> JoshutoConfig {
        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            None => (1, 3, 4),
        };

        let scroll_offset: usize = self.scroll_offset.unwrap_or(6);
        let tilde_in_titlebar: bool = self.tilde_in_titlebar.unwrap_or(true);
        let show_preview: bool = self.show_preview.unwrap_or(true);
        let max_preview_size: u64 = self.max_preview_size.unwrap_or(2 * 1024 * 1024);

        let sort_method: sort::SortType = match self.sort_method {
            Some(s) => match s.as_str() {
                "mtime" => sort::SortType::SortMtime,
                _ => sort::SortType::SortNatural,
            },
            _ => sort::SortType::SortNatural,
        };

        let show_hidden: bool;
        let case_sensitive: bool;
        let reverse: bool;
        let directories_first: bool;

        match self.sort_option {
            Some(s) => {
                show_hidden = s.show_hidden.unwrap_or(false);
                case_sensitive = s.case_sensitive.unwrap_or(false);
                reverse = s.reverse.unwrap_or(false);
                directories_first = s.directories_first.unwrap_or(true);
            }
            None => {
                show_hidden = false;
                case_sensitive = false;
                reverse = false;
                directories_first = true;
            }
        }

        let sort_option = sort::SortOption {
            show_hidden,
            directories_first,
            case_sensitive,
            reverse,
            sort_method,
        };

        JoshutoConfig {
            scroll_offset,
            tilde_in_titlebar,
            show_preview,
            max_preview_size,
            column_ratio,
            sort_option,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoConfig {
    pub scroll_offset: usize,
    pub tilde_in_titlebar: bool,
    pub show_preview: bool,
    pub max_preview_size: u64,
    pub sort_option: sort::SortOption,
    pub column_ratio: (usize, usize, usize),
}

impl ConfigStructure for JoshutoConfig {
    fn get_config() -> Self {
        parse_config_file::<JoshutoRawConfig, JoshutoConfig>(CONFIG_FILE)
            .unwrap_or_else(JoshutoConfig::default)
    }
}

impl std::default::Default for JoshutoConfig {
    fn default() -> Self {
        let sort_option = sort::SortOption {
            show_hidden: false,
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_method: sort::SortType::SortNatural,
        };

        JoshutoConfig {
            scroll_offset: 6,
            tilde_in_titlebar: true,
            show_preview: true,
            max_preview_size: 2 * 1024 * 1024,
            sort_option,
            column_ratio: (1, 3, 4),
        }
    }
}
