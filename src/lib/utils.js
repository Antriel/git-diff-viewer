export function debounce(fn, ms) {
  let timeoutId;
  return (...args) => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => fn(...args), ms);
  };
}

export function escapeHtml(str) {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

export function parseHunkHeader(header) {
  const match = header.match(/@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@/);
  return match
    ? {
      oldStart: parseInt(match[1], 10),
      newStart: parseInt(match[2], 10),
    }
    : { oldStart: 1, newStart: 1 };
}

export function formatRelativeTime(isoString) {
  if (isoString === "unknown") return "unknown";

  const now = new Date();
  const modifiedDate = new Date(isoString);
  const diffMs = now.getTime() - modifiedDate.getTime();

  const rtf = new Intl.RelativeTimeFormat("en", { numeric: "auto" });

  const diffSeconds = Math.floor(diffMs / 1000);
  const diffMinutes = Math.floor(diffSeconds / 60);
  const diffHours = Math.floor(diffMinutes / 60);
  const diffDays = Math.floor(diffHours / 24);
  const diffWeeks = Math.floor(diffDays / 7);
  const diffMonths = Math.floor(diffDays / 30);
  const diffYears = Math.floor(diffDays / 365);

  if (Math.abs(diffSeconds) < 60) return rtf.format(0, "second");
  if (Math.abs(diffMinutes) < 60) return rtf.format(-diffMinutes, "minute");
  if (Math.abs(diffHours) < 24) return rtf.format(-diffHours, "hour");
  if (Math.abs(diffDays) < 7) return rtf.format(-diffDays, "day");
  if (Math.abs(diffWeeks) < 4) return rtf.format(-diffWeeks, "week");
  if (Math.abs(diffMonths) < 12) return rtf.format(-diffMonths, "month");
  return rtf.format(-diffYears, "year");
}

export function formatLocalTime(isoString) {
  if (isoString === "unknown") return "Unknown";
  const date = new Date(isoString);
  return date.toLocaleString();
}

import hljs from "highlight.js";

export function applySyntaxHighlighting(content, fileName) {
  try {
    // Get file extension for language detection
    const ext = fileName.split(".").pop()?.toLowerCase();
    const language = hljs.getLanguage(ext);

    if (language) {
      const highlighted = hljs.highlight(content, { language: ext });
      return highlighted.value;
    } else {
      // Try auto-detection if specific language not found
      const highlighted = hljs.highlightAuto(content);
      return highlighted.value;
    }
  } catch (error) {
    console.warn("Syntax highlighting failed:", error);
    return escapeHtml(content);
  }
}

