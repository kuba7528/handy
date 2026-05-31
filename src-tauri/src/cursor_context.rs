/// Detect text context before the caret in the currently focused field.
/// Used to decide whether pasted transcription should start with a capital letter.

/// Returns the text before the text caret in the focused control, if readable.
#[cfg(target_os = "windows")]
pub fn get_text_before_cursor() -> Option<String> {
    use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        GetClassNameW, GetForegroundWindow, GetGUIThreadInfo, GetWindowThreadProcessId,
        SendMessageW, GUITHREADINFO,
    };

    const EM_GETSEL: u32 = 0x00B0;
    const WM_GETTEXT: u32 = 0x000D;
    const WM_GETTEXTLENGTH: u32 = 0x000E;

    unsafe {
        let foreground = GetForegroundWindow();
        if foreground == HWND::default() {
            return None;
        }

        let thread_id = GetWindowThreadProcessId(foreground, None);
        let mut info = GUITHREADINFO {
            cbSize: std::mem::size_of::<GUITHREADINFO>() as u32,
            ..Default::default()
        };
        if GetGUIThreadInfo(thread_id, &mut info).is_err() {
            return None;
        }

        let focus = info.hwndFocus;
        if focus == HWND::default() {
            return None;
        }

        let mut class_name = [0u16; 256];
        let class_len = GetClassNameW(focus, &mut class_name);
        if class_len == 0 {
            return None;
        }
        let class = String::from_utf16_lossy(&class_name[..class_len as usize]);
        let class_lower = class.to_ascii_lowercase();
        if !class_lower.contains("edit") {
            return None;
        }

        let mut start: u32 = 0;
        let mut end: u32 = 0;
        SendMessageW(
            focus,
            EM_GETSEL,
            Some(WPARAM(&mut start as *mut u32 as usize)),
            Some(LPARAM(&mut end as *mut u32 as isize)),
        );

        let text_len = SendMessageW(focus, WM_GETTEXTLENGTH, None, None).0 as i32;
        if text_len < 0 {
            return None;
        }

        if text_len == 0 {
            return Some(String::new());
        }

        let mut buffer = vec![0u16; text_len as usize + 1];
        SendMessageW(
            focus,
            WM_GETTEXT,
            Some(WPARAM(buffer.len())),
            Some(LPARAM(buffer.as_mut_ptr() as isize)),
        );
        let full_text = String::from_utf16_lossy(&buffer[..text_len as usize]);
        let cursor = (start as usize).min(full_text.len());
        Some(full_text[..cursor].to_string())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_text_before_cursor() -> Option<String> {
    None
}

/// Whether pasted text should begin with an uppercase letter.
///
/// Capitalize when the caret is at the start of the field or immediately after
/// sentence-ending punctuation (`.`, `!`, `?`, optionally followed by whitespace).
/// Otherwise keep the first letter lowercase so mid-sentence dictation flows naturally.
pub fn should_capitalize_pasted_text() -> bool {
    match get_text_before_cursor() {
        Some(context) => {
            let trimmed = context.trim_end();
            if trimmed.is_empty() {
                return true;
            }
            trimmed
                .chars()
                .last()
                .is_some_and(|c| matches!(c, '.' | '!' | '?'))
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::should_capitalize_pasted_text;

    #[test]
    fn capitalize_decision_without_cursor_context_defaults_to_lowercase() {
        #[cfg(not(target_os = "windows"))]
        assert!(!should_capitalize_pasted_text());
    }
}
