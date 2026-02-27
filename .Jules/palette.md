# 2024-05-24 - Overlay Accessibility & Visual Feedback
**Learning:** Frameless overlay windows in Tkinter often lack accessible titles, making them invisible or confusing to screen reader users who navigate by window name.
**Action:** Always explicitly set `root.title()` even for `overrideredirect(True)` windows to ensure they are identifiable in the accessibility tree.

# 2024-05-24 - Visual State Communication
**Learning:** Using color alone to convey state (e.g., Green for Listening, Red for Recording) excludes colorblind users. Adding semantic emojis (👂, 🔴) provides a secondary visual cue that is universally understood and adds a layer of delight.
**Action:** Pair all color-coded status updates with a relevant icon or emoji to improve scanability and accessibility.
