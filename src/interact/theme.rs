use crossterm::style::Stylize;
use dialoguer::theme::Theme;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::internal::utils::wrap_text;
const ERR_SYMBOL: &str = "X";
const PROMPT_SYMBOL: &str = "?";

pub struct AmeTheme;

impl AmeTheme {
    pub fn get() -> &'static Self {
        static AME_THEME: AmeTheme = AmeTheme;
        &AME_THEME
    }
}

impl Theme for AmeTheme {
    fn format_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        let prompt = wrap_text(prompt, 4).join("\n");
        write!(f, "{} {}:", PROMPT_SYMBOL.magenta(), prompt.bold())
    }

    fn format_error(&self, f: &mut dyn std::fmt::Write, err: &str) -> std::fmt::Result {
        write!(f, "{} error: {}", ERR_SYMBOL.red(), err)
    }

    fn format_confirm_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> std::fmt::Result {
        let prompt = wrap_text(prompt, 4).join("\n");
        if !prompt.is_empty() {
            write!(f, "{} {} ", PROMPT_SYMBOL.magenta(), &prompt.bold())?;
        }
        match default {
            None => write!(f, "[y/n] ")?,
            Some(true) => write!(f, "[{}/n] ", "Y".bold())?,
            Some(false) => write!(f, "[y/{}] ", "N".bold())?,
        }
        Ok(())
    }

    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> std::fmt::Result {
        let prompt = wrap_text(prompt, 4).join("\n");
        let selection = selection.map(|b| if b { "yes" } else { "no" });

        match selection {
            Some(selection) if prompt.is_empty() => {
                write!(f, "  {}", selection.italic())
            }
            Some(selection) => {
                write!(f, "  {} {}", &prompt.bold(), selection.italic())
            }
            None if prompt.is_empty() => Ok(()),
            None => {
                write!(f, "  {}", &prompt.bold())
            }
        }
    }

    fn format_input_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> std::fmt::Result {
        match default {
            Some(default) if prompt.is_empty() => {
                write!(f, "{} [{}]: ", PROMPT_SYMBOL.magenta(), default)
            }
            Some(default) => write!(
                f,
                "{} {} [{}]: ",
                PROMPT_SYMBOL.magenta(),
                prompt.bold(),
                default
            ),
            None => write!(f, "{} {}: ", PROMPT_SYMBOL.magenta(), prompt.bold()),
        }
    }

    fn format_input_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} {}: {}",
            PROMPT_SYMBOL.magenta(),
            prompt.bold(),
            sel.italic()
        )
    }

    fn format_password_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt(f, prompt, None)
    }

    fn format_password_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, "[hidden]")
    }

    fn format_select_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, sel)
    }

    fn format_multi_select_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_sort_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        write!(f, "{}: ", prompt.bold())?;
        if selections.is_empty() {
            write!(f, "{}", "No selections".italic())?;
        } else {
            for (idx, sel) in selections.iter().enumerate() {
                write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
            }
        }
        Ok(())
    }

    fn format_sort_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        self.format_multi_select_prompt_selection(f, prompt, selections)
    }

    fn format_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        active: bool,
    ) -> std::fmt::Result {
        write!(f, "{} {}", if active { ">" } else { " " }, text)
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> std::fmt::Result {
        let active_symbol = if active { ">" } else { " " };
        let checked_symbol = if checked { "x" } else { " " }.magenta();
        write!(f, "{active_symbol} [{checked_symbol}] {text}")
    }

    fn format_sort_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        picked: bool,
        active: bool,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match (picked, active) {
                (true, true) => "> [x]",
                (false, true) => "> [ ]",
                (_, false) => "  [ ]",
            },
            text
        )
    }

    fn format_fuzzy_select_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        search_term: &str,
        cursor_pos: usize,
    ) -> std::fmt::Result {
        if !prompt.is_empty() {
            write!(f, "{} {} ", PROMPT_SYMBOL.magenta(), prompt.bold())?;
        }

        if cursor_pos < search_term.len() {
            let st_head = search_term[0..cursor_pos].to_string();
            let st_tail = search_term[cursor_pos..search_term.len()].to_string();
            let st_cursor = "|".to_string();
            write!(f, "{}{}{}", st_head, st_cursor, st_tail)
        } else {
            let cursor = "|".to_string();
            write!(f, "{}{}", search_term, cursor)
        }
    }

    fn format_fuzzy_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        active: bool,
        highlight_matches: bool,
        matcher: &SkimMatcherV2,
        search_term: &str,
    ) -> std::fmt::Result {
        write!(f, "{} ", if active { ">" } else { " " }.magenta().bold())?;

        if highlight_matches {
            if let Some((_score, indices)) = matcher.fuzzy_indices(text, search_term) {
                for (idx, c) in text.chars().into_iter().enumerate() {
                    if indices.contains(&idx) {
                        write!(f, "{}", c.bold())?;
                    } else {
                        write!(f, "{}", c)?;
                    }
                }

                return Ok(());
            }
        }

        write!(f, "{}", text)
    }
}
