//! Small text-truncation helpers shared by [`crate::model::cell::Cell`]'s
//! eager `truncate`/`truncate_with` and the renderer's final pass that
//! truncates each cell to its resolved column width.

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Truncates `text` to fit within `max_width` terminal columns, replacing
/// the cut-off tail with `ellipsis` (only reachable once a configured
/// max width narrows it below the content's natural width).
pub(crate) fn truncate_with_ellipsis(text: &str, max_width: usize, ellipsis: &str) -> String {
    if text.width() <= max_width {
        return text.to_string();
    }
    let ellipsis_width = ellipsis.width();
    if max_width <= ellipsis_width {
        return take_by_width(text, max_width);
    }
    let mut truncated = take_by_width(text, max_width - ellipsis_width);
    truncated.push_str(ellipsis);
    truncated
}

/// Takes as many leading characters of `text` as fit within `max_width`
/// terminal columns (unicode-width aware, so wide/CJK characters aren't cut
/// in half).
pub(crate) fn take_by_width(text: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut width_so_far = 0;
    for ch in text.chars() {
        let ch_width = ch.width().unwrap_or(0);
        if width_so_far + ch_width > max_width {
            break;
        }
        result.push(ch);
        width_so_far += ch_width;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_shorter_than_max_width_is_unchanged() {
        assert_eq!(truncate_with_ellipsis("hi", 10, "..."), "hi");
    }

    #[test]
    fn text_longer_than_max_width_is_truncated_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("a very long value", 10, "..."), "a very ...");
    }

    #[test]
    fn custom_ellipsis_is_used_instead_of_the_default() {
        assert_eq!(truncate_with_ellipsis("a very long value", 8, "…"), "a very …");
    }

    #[test]
    fn max_width_too_small_for_the_ellipsis_falls_back_to_hard_truncation() {
        assert_eq!(truncate_with_ellipsis("hello", 2, "..."), "he");
    }
}
