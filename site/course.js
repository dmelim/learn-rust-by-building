"use strict";

const COURSE_SECTIONS = [
  {
    title: "Start here",
    items: [
      { title: "Course overview", path: "course/README.md" },
      { title: "How to use this course", path: "course/00-how-to-use-this-course.md" },
    ],
  },
  {
    title: "Foundations",
    items: [
      { title: "Module overview", path: "course/01-foundations/README.md" },
      { title: "Cargo and the compiler", path: "course/01-foundations/01-cargo-and-the-compiler.md" },
      { title: "Bindings and types", path: "course/01-foundations/02-bindings-and-types.md" },
      { title: "Functions and expressions", path: "course/01-foundations/03-functions-and-expressions.md" },
      { title: "Decisions and repetition", path: "course/01-foundations/04-decisions-and-repetition.md" },
      { title: "Terminal input", path: "course/01-foundations/05-terminal-input.md" },
      { title: "Practical: delivery estimator", path: "course/01-foundations/06-practical-delivery-estimator.md" },
    ],
  },
  {
    title: "Ownership and borrowing",
    items: [
      { title: "Module overview", path: "course/02-ownership/README.md" },
      { title: "Owned strings", path: "course/02-ownership/01-owned-strings.md" },
      { title: "Moves and clones", path: "course/02-ownership/02-moves-and-clones.md" },
      { title: "Shared borrowing", path: "course/02-ownership/03-shared-borrowing.md" },
      { title: "Mutable borrowing", path: "course/02-ownership/04-mutable-borrowing.md" },
      { title: "String slices", path: "course/02-ownership/05-string-slices.md" },
      { title: "Practical: customer data", path: "course/02-ownership/06-practical-customer-data.md" },
    ],
  },
  {
    title: "Domain modeling",
    items: [
      { title: "Module overview", path: "course/03-domain-modeling/README.md" },
      { title: "Structs", path: "course/03-domain-modeling/01-structs.md" },
      { title: "Methods", path: "course/03-domain-modeling/02-methods.md" },
      { title: "Enums", path: "course/03-domain-modeling/03-enums.md" },
      { title: "Optional data", path: "course/03-domain-modeling/04-option.md" },
      { title: "Pattern matching", path: "course/03-domain-modeling/05-pattern-matching.md" },
      { title: "Practical: typed orders", path: "course/03-domain-modeling/06-practical-orders.md" },
    ],
  },
  {
    title: "Useful collections",
    items: [
      { title: "Module overview", path: "course/04-collections/README.md" },
      { title: "Vectors", path: "course/04-collections/01-vectors.md" },
      { title: "Iteration", path: "course/04-collections/02-iteration.md" },
      { title: "Strings in collections", path: "course/04-collections/03-strings.md" },
      { title: "Hash maps", path: "course/04-collections/04-hash-maps.md" },
      { title: "Queue operations", path: "course/04-collections/05-queue-operations.md" },
      { title: "Practical: delivery queue", path: "course/04-collections/06-practical-delivery-queue.md" },
    ],
  },
  {
    title: "Reliable structure",
    items: [
      { title: "Module overview", path: "course/05-reliable-structure/README.md" },
      { title: "Packages and crates", path: "course/05-reliable-structure/01-packages-and-crates.md" },
      { title: "Modules and visibility", path: "course/05-reliable-structure/02-modules-and-visibility.md" },
      { title: "Recoverable errors", path: "course/05-reliable-structure/03-recoverable-errors.md" },
      { title: "Error propagation", path: "course/05-reliable-structure/04-error-propagation.md" },
      { title: "Application boundaries", path: "course/05-reliable-structure/05-application-boundaries.md" },
      { title: "Practical: reliable app", path: "course/05-reliable-structure/06-practical-reliable-app.md" },
    ],
  },
  {
    title: "Tests, traits, and files",
    items: [
      { title: "Module overview", path: "course/06-tests-traits-files/README.md" },
      { title: "Generics", path: "course/06-tests-traits-files/01-generics.md" },
      { title: "Traits", path: "course/06-tests-traits-files/02-traits.md" },
      { title: "Lifetimes at the boundary", path: "course/06-tests-traits-files/03-lifetimes.md" },
      { title: "Testing public behavior", path: "course/06-tests-traits-files/04-testing.md" },
      { title: "File I/O", path: "course/06-tests-traits-files/05-file-io.md" },
      { title: "Practical: persistence", path: "course/06-tests-traits-files/06-practical-persistence.md" },
    ],
  },
  {
    title: "Capstone",
    items: [{ title: "Dispatch Desk CLI", path: "course/capstone/README.md" }],
  },
];

const DEFAULT_PATH = "course/README.md";
const PROGRESS_KEY = "learn-rust-course-progress";
const THEME_KEY = "learn-rust-course-theme";
const ALL_ITEMS = COURSE_SECTIONS.flatMap((section) =>
  section.items.map((item) => ({ ...item, section: section.title })),
);

const elements = {
  content: document.querySelector("#lesson-content"),
  loading: document.querySelector("#lesson-loading"),
  navigation: document.querySelector("#course-navigation"),
  outline: document.querySelector("#page-outline"),
  search: document.querySelector("#course-search"),
  progress: document.querySelector("#progress-copy"),
  headerProgress: document.querySelector("#header-progress"),
  theme: document.querySelector("#theme-button"),
  menu: document.querySelector("#menu-button"),
  sidebar: document.querySelector("#course-sidebar"),
  scrim: document.querySelector("#mobile-scrim"),
};

let currentPath = DEFAULT_PATH;
let completed = loadCompleted();
let diagramRenderGeneration = 0;

function escapeHtml(value) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

function normalizePath(path) {
  const parts = [];
  for (const part of path.split("/")) {
    if (!part || part === ".") continue;
    if (part === "..") parts.pop();
    else parts.push(part);
  }
  return parts.join("/");
}

function resolveRelativePath(sourcePath, target) {
  const cleanTarget = target.split("#")[0];
  const directory = sourcePath.split("/").slice(0, -1).join("/");
  return normalizePath(`${directory}/${cleanTarget}`);
}

function safeHref(rawHref, sourcePath) {
  const href = rawHref.replaceAll("&amp;", "&");
  if (/^https?:\/\//i.test(href) || href.startsWith("mailto:")) {
    return { href, external: true };
  }
  if (href.startsWith("#")) {
    return { href: `#/${sourcePath}${href}`, external: false };
  }

  const [target, fragment = ""] = href.split("#");
  const resolved = resolveRelativePath(sourcePath, target);
  if (resolved.endsWith(".md")) {
    return { href: `#/${resolved}${fragment ? `#${fragment}` : ""}`, external: false };
  }
  return { href: resolved, external: true };
}

function renderInline(source, sourcePath) {
  const codeTokens = [];
  let text = source.replace(/`([^`]+)`/g, (_, code) => {
    const token = `@@CODE${codeTokens.length}@@`;
    codeTokens.push(`<code>${escapeHtml(code)}</code>`);
    return token;
  });

  text = escapeHtml(text);
  text = text.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_, label, rawHref) => {
    const link = safeHref(rawHref, sourcePath);
    const attributes = link.external ? ' target="_blank" rel="noreferrer"' : "";
    return `<a href="${escapeHtml(link.href)}"${attributes}>${label}</a>`;
  });
  text = text.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
  text = text.replace(/(^|\s)\*([^*]+)\*/g, "$1<em>$2</em>");
  codeTokens.forEach((code, index) => {
    text = text.replace(`@@CODE${index}@@`, code);
  });
  return text;
}

function slugify(value, usedIds) {
  const base = value
    .toLowerCase()
    .replace(/<[^>]+>/g, "")
    .replace(/[^a-z0-9\s-]/g, "")
    .trim()
    .replace(/\s+/g, "-") || "section";
  let slug = base;
  let suffix = 2;
  while (usedIds.has(slug)) {
    slug = `${base}-${suffix}`;
    suffix += 1;
  }
  usedIds.add(slug);
  return slug;
}

function isTableSeparator(line) {
  return /^\s*\|?\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)+\|?\s*$/.test(line);
}

function tableCells(line) {
  return line
    .trim()
    .replace(/^\|/, "")
    .replace(/\|$/, "")
    .split("|")
    .map((cell) => cell.trim());
}

function isBlockStart(lines, index) {
  const line = lines[index] ?? "";
  const next = lines[index + 1] ?? "";
  return (
    !line.trim() ||
    /^#{1,4}\s+/.test(line) ||
    /^```/.test(line) ||
    /^>\s?/.test(line) ||
    /^\s*(?:[-*]|\d+\.)\s+/.test(line) ||
    /^<\/?(?:details|summary)>/.test(line.trim()) ||
    (line.includes("|") && isTableSeparator(next))
  );
}

function renderMarkdown(markdown, sourcePath) {
  const source = markdown.replaceAll("\r", "").replace(/[—–]/g, "-");
  const lines = source.split("\n");
  const html = [];
  const outline = [];
  const usedIds = new Set();
  let diagramIndex = 0;
  let index = 0;

  while (index < lines.length) {
    const line = lines[index];
    if (!line.trim()) {
      index += 1;
      continue;
    }

    if (line.startsWith("```")) {
      const language = line.slice(3).trim() || "text";
      const code = [];
      index += 1;
      while (index < lines.length && !lines[index].startsWith("```")) {
        code.push(lines[index]);
        index += 1;
      }
      index += 1;
      const sourceCode = code.join("\n");
      if (language.toLowerCase() === "mermaid") {
        diagramIndex += 1;
        html.push(
          `<figure class="diagram-block" data-diagram-index="${diagramIndex}">` +
            `<div class="diagram-toolbar"><span>Flowchart</span><span>Interactive course map</span></div>` +
            `<div class="diagram-canvas is-loading" role="status" aria-live="polite">Rendering diagram...</div>` +
            `<details class="diagram-source"><summary>View diagram source</summary>` +
            `<div class="diagram-source-toolbar"><span>mermaid</span>` +
            `<button class="copy-button" type="button">Copy</button></div>` +
            `<pre><code>${escapeHtml(sourceCode)}</code></pre></details></figure>`,
        );
        continue;
      }
      html.push(
        `<div class="code-block"><div class="code-toolbar"><span>${escapeHtml(language)}</span>` +
          `<button class="copy-button" type="button">Copy</button></div>` +
          `<pre><code>${escapeHtml(sourceCode)}</code></pre></div>`,
      );
      continue;
    }

    const heading = line.match(/^(#{1,4})\s+(.+)$/);
    if (heading) {
      const level = heading[1].length;
      const content = renderInline(heading[2], sourcePath);
      const id = slugify(heading[2], usedIds);
      html.push(`<h${level} id="${id}">${content}</h${level}>`);
      if (level === 2 || level === 3) outline.push({ level, title: heading[2], id });
      index += 1;
      continue;
    }

    if (line.includes("|") && isTableSeparator(lines[index + 1] ?? "")) {
      const headers = tableCells(line);
      const rows = [];
      index += 2;
      while (index < lines.length && lines[index].includes("|") && lines[index].trim()) {
        rows.push(tableCells(lines[index]));
        index += 1;
      }
      html.push(
        `<div class="table-wrap"><table><thead><tr>${headers
          .map((cell) => `<th>${renderInline(cell, sourcePath)}</th>`)
          .join("")}</tr></thead><tbody>${rows
          .map(
            (row) =>
              `<tr>${row.map((cell) => `<td>${renderInline(cell, sourcePath)}</td>`).join("")}</tr>`,
          )
          .join("")}</tbody></table></div>`,
      );
      continue;
    }

    if (/^>\s?/.test(line)) {
      const quote = [];
      while (index < lines.length && /^>\s?/.test(lines[index])) {
        quote.push(lines[index].replace(/^>\s?/, ""));
        index += 1;
      }
      html.push(`<blockquote><p>${renderInline(quote.join(" "), sourcePath)}</p></blockquote>`);
      continue;
    }

    const listItem = line.match(/^\s*([-*]|\d+\.)\s+(.+)$/);
    if (listItem) {
      const ordered = /\d+\./.test(listItem[1]);
      const items = [];
      while (index < lines.length) {
        const match = lines[index].match(/^\s*([-*]|\d+\.)\s+(.+)$/);
        if (!match || /\d+\./.test(match[1]) !== ordered) break;
        let item = match[2];
        index += 1;
        while (
          index < lines.length &&
          lines[index].trim() &&
          !/^\s*([-*]|\d+\.)\s+/.test(lines[index]) &&
          !isBlockStart(lines, index)
        ) {
          item += ` ${lines[index].trim()}`;
          index += 1;
        }
        items.push(item);
      }
      const tag = ordered ? "ol" : "ul";
      html.push(`<${tag}>${items.map((item) => `<li>${renderInline(item, sourcePath)}</li>`).join("")}</${tag}>`);
      continue;
    }

    const trimmed = line.trim();
    if (trimmed === "<details>" || trimmed === "</details>") {
      html.push(trimmed);
      index += 1;
      continue;
    }
    const summary = trimmed.match(/^<summary>(.*)<\/summary>$/);
    if (summary) {
      html.push(`<summary>${renderInline(summary[1], sourcePath)}</summary>`);
      index += 1;
      continue;
    }

    const paragraph = [line.trim()];
    index += 1;
    while (index < lines.length && !isBlockStart(lines, index)) {
      paragraph.push(lines[index].trim());
      index += 1;
    }
    html.push(`<p>${renderInline(paragraph.join(" "), sourcePath)}</p>`);
  }

  return { html: html.join("\n"), outline };
}

function diagramTheme() {
  const styles = getComputedStyle(document.documentElement);
  const token = (name) => styles.getPropertyValue(name).trim();
  return {
    background: token("--surface"),
    primaryColor: token("--accent-soft"),
    primaryTextColor: token("--ink"),
    primaryBorderColor: token("--accent"),
    secondaryColor: token("--surface-strong"),
    secondaryTextColor: token("--ink"),
    secondaryBorderColor: token("--line-strong"),
    tertiaryColor: token("--surface"),
    tertiaryTextColor: token("--ink"),
    tertiaryBorderColor: token("--line"),
    lineColor: token("--muted"),
    textColor: token("--ink"),
    mainBkg: token("--accent-soft"),
    nodeBorder: token("--accent"),
    clusterBkg: token("--surface-strong"),
    clusterBorder: token("--line-strong"),
    edgeLabelBackground: token("--surface"),
    fontFamily: token("--font-sans"),
  };
}

async function renderMermaidDiagrams() {
  const figures = [...elements.content.querySelectorAll(".diagram-block")];
  if (!figures.length) return;

  const generation = ++diagramRenderGeneration;
  if (!window.mermaid) {
    figures.forEach((figure) => {
      const canvas = figure.querySelector(".diagram-canvas");
      canvas.className = "diagram-canvas has-error";
      canvas.setAttribute("role", "alert");
      canvas.textContent = "The diagram renderer could not load. The source is available below.";
    });
    return;
  }

  window.mermaid.initialize({
    startOnLoad: false,
    securityLevel: "strict",
    theme: "base",
    themeVariables: diagramTheme(),
    flowchart: { htmlLabels: true, useMaxWidth: true, curve: "basis" },
  });

  for (const [index, figure] of figures.entries()) {
    if (generation !== diagramRenderGeneration) return;
    const canvas = figure.querySelector(".diagram-canvas");
    const source = figure.querySelector(".diagram-source code").textContent;
    canvas.className = "diagram-canvas is-loading";
    canvas.setAttribute("role", "status");
    canvas.textContent = "Rendering diagram...";
    try {
      const result = await window.mermaid.render(`course-diagram-${generation}-${index}`, source);
      if (generation !== diagramRenderGeneration || !canvas.isConnected) return;
      canvas.innerHTML = result.svg;
      canvas.className = "diagram-canvas";
      canvas.setAttribute("role", "img");
      canvas.setAttribute("aria-label", "Rendered course flowchart");
      result.bindFunctions?.(canvas);
    } catch (error) {
      if (generation !== diagramRenderGeneration || !canvas.isConnected) return;
      canvas.className = "diagram-canvas has-error";
      canvas.setAttribute("role", "alert");
      canvas.textContent = "This flowchart could not be rendered. Check its source below.";
      console.error("Could not render Mermaid diagram", error);
    }
  }
}

function loadCompleted() {
  try {
    return new Set(JSON.parse(localStorage.getItem(PROGRESS_KEY) || "[]"));
  } catch {
    return new Set();
  }
}

function saveCompleted() {
  try {
    localStorage.setItem(PROGRESS_KEY, JSON.stringify([...completed]));
  } catch {
    elements.progress.textContent = "Progress could not be saved in this browser.";
  }
}

function renderNavigation() {
  elements.navigation.innerHTML = COURSE_SECTIONS.map(
    (section, sectionIndex) => `
      <details class="nav-section" data-section="${sectionIndex}" ${sectionIndex < 2 ? "open" : ""}>
        <summary>${escapeHtml(section.title)}</summary>
        <ol>
          ${section.items
            .map(
              (item, itemIndex) => `
                <li data-search="${escapeHtml(`${section.title} ${item.title}`.toLowerCase())}">
                  <a class="nav-link" href="#/${item.path}" data-path="${item.path}">
                    <span class="nav-index">${String(itemIndex + 1).padStart(2, "0")}</span>
                    <span>${escapeHtml(item.title)}</span>
                  </a>
                </li>`,
            )
            .join("")}
        </ol>
      </details>`,
  ).join("");
  updateNavigationState();
}

function updateNavigationState() {
  document.querySelectorAll(".nav-link").forEach((link) => {
    const path = link.dataset.path;
    link.toggleAttribute("aria-current", path === currentPath);
    if (path === currentPath) link.setAttribute("aria-current", "page");
    link.classList.toggle("is-complete", completed.has(path));
    if (path === currentPath) link.closest("details").open = true;
  });

  const completeCount = ALL_ITEMS.filter((item) => completed.has(item.path)).length;
  elements.headerProgress.textContent = `${completeCount} of ${ALL_ITEMS.length} complete`;
  elements.progress.textContent = completeCount
    ? `${completeCount} lessons complete. Progress stays in this browser.`
    : "Your progress stays in this browser.";
}

function renderOutline(headings) {
  elements.outline.innerHTML = headings.length
    ? headings
        .map(
          (heading) =>
            `<a class="outline-h${heading.level}" href="#/${currentPath}#${heading.id}">${escapeHtml(
              heading.title,
            )}</a>`,
        )
        .join("")
    : '<span class="progress-copy">No sections in this page.</span>';
}

function practicalInfo(path) {
  if (path === "course/capstone/README.md") {
    return {
      starter: "course/capstone/starter/src/lib.rs",
      command: "cargo test -p dispatch_desk_starter",
    };
  }
  if (!/\/\d{2}-practical-[^/]+\.md$/.test(path)) return null;
  const directory = path.split("/").slice(0, -1).join("/");
  const moduleSlug = directory.split("/").pop();
  return {
    starter: `${directory}/practical/starter/src/main.rs`,
    command: `cargo test -p ${starterPackage(moduleSlug)}`,
  };
}

function starterPackage(moduleSlug) {
  const packages = {
    "01-foundations": "delivery_estimator_starter",
    "02-ownership": "ownership_customer_starter",
    "03-domain-modeling": "domain_orders_starter",
    "04-collections": "collections_queue_starter",
    "05-reliable-structure": "reliable_app_starter",
    "06-tests-traits-files": "persistent_dispatch_starter",
  };
  return packages[moduleSlug] || "starter";
}

function practicalCallout(info) {
  return `
    <section class="practical-callout" aria-labelledby="build-in-editor">
      <div>
        <h2 id="build-in-editor">Move into your code editor</h2>
        <p>Open <code>${escapeHtml(info.starter)}</code>, complete the TODOs, then run <code>${escapeHtml(
          info.command,
        )}</code>.</p>
      </div>
      <div class="practical-actions">
        <a class="primary-button" href="${info.starter}" target="_blank">Open starter file</a>
        <button class="secondary-button copy-path-button" type="button" data-copy="${escapeHtml(
          info.starter,
        )}">Copy path</button>
      </div>
    </section>`;
}

function lessonFooter(path) {
  const index = ALL_ITEMS.findIndex((item) => item.path === path);
  if (index < 0) return "";
  const previous = ALL_ITEMS[index - 1];
  const next = ALL_ITEMS[index + 1];
  const isComplete = completed.has(path);
  return `
    <footer class="lesson-footer">
      ${previous ? `<a href="#/${previous.path}">Previous: ${escapeHtml(previous.title)}</a>` : "<span></span>"}
      <button class="complete-button ${isComplete ? "is-complete" : ""}" type="button" data-complete="${path}">
        ${isComplete ? "Completed" : "Mark complete"}
      </button>
      ${next ? `<a href="#/${next.path}">Next: ${escapeHtml(next.title)}</a>` : "<span></span>"}
    </footer>`;
}

function readingTime(markdown) {
  const words = markdown.replace(/```[\s\S]*?```/g, "").trim().split(/\s+/).filter(Boolean).length;
  return Math.max(1, Math.ceil(words / 220));
}

async function loadLesson() {
  const route = decodeURIComponent(location.hash.replace(/^#\/?/, ""));
  const [routePath, fragment = ""] = route.split("#");
  currentPath = routePath || DEFAULT_PATH;
  elements.loading.hidden = false;
  elements.content.hidden = true;
  elements.content.innerHTML = "";
  updateNavigationState();
  closeMobileMenu();

  try {
    const response = await fetch(currentPath, { cache: "no-store" });
    if (!response.ok) throw new Error(`The server returned ${response.status}.`);
    const markdown = await response.text();
    const rendered = renderMarkdown(markdown, currentPath);
    const item = ALL_ITEMS.find((candidate) => candidate.path === currentPath);
    const section = item?.section || "Course notes";
    const practical = practicalInfo(currentPath);
    const welcome =
      currentPath === DEFAULT_PATH
        ? '<div class="welcome-actions"><a class="primary-button" href="#/course/00-how-to-use-this-course.md">Start the course</a><span>Six modules, one growing project.</span></div>'
        : "";
    const lead = welcome || (practical ? practicalCallout(practical) : "");
    const articleHtml = lead ? rendered.html.replace("</h1>", `</h1>${lead}`) : rendered.html;

    elements.content.innerHTML = `
      <div class="lesson-meta"><span>${escapeHtml(section)}</span><span>${readingTime(markdown)} min read</span></div>
      ${articleHtml}
      ${lessonFooter(currentPath)}`;
    renderOutline(rendered.outline);
    document.title = `${item?.title || "Learn Rust"} | Learn Rust by Building`;
    elements.loading.hidden = true;
    elements.content.hidden = false;
    void renderMermaidDiagrams();
    document.querySelector("#lesson").focus({ preventScroll: true });
    window.scrollTo({ top: 0, behavior: "auto" });
    if (fragment) {
      window.requestAnimationFrame(() => document.getElementById(fragment)?.scrollIntoView());
    }
  } catch (error) {
    elements.loading.hidden = true;
    elements.content.hidden = false;
    elements.outline.innerHTML = "";
    elements.content.innerHTML = `
      <div class="error-state">
        <h1>The lesson could not load</h1>
        <p>${escapeHtml(error.message)}</p>
        <p>Serve the repository over HTTP with <code>bash scripts/serve-course.sh</code>, then open <code>http://localhost:8000</code>.</p>
      </div>`;
  }
}

function copyText(value, button) {
  navigator.clipboard.writeText(value).then(() => {
    const original = button.textContent;
    button.textContent = "Copied";
    window.setTimeout(() => {
      button.textContent = original;
    }, 1200);
  });
}

function handleContentClick(event) {
  const copyButton = event.target.closest(".copy-button");
  if (copyButton) {
    const code = copyButton.closest(".code-block, .diagram-source")?.querySelector("code");
    if (code) copyText(code.textContent, copyButton);
    return;
  }

  const pathButton = event.target.closest(".copy-path-button");
  if (pathButton) {
    copyText(pathButton.dataset.copy, pathButton);
    return;
  }

  const completeButton = event.target.closest("[data-complete]");
  if (completeButton) {
    const path = completeButton.dataset.complete;
    if (completed.has(path)) completed.delete(path);
    else completed.add(path);
    saveCompleted();
    updateNavigationState();
    completeButton.classList.toggle("is-complete", completed.has(path));
    completeButton.textContent = completed.has(path) ? "Completed" : "Mark complete";
  }
}

function filterNavigation(query) {
  const normalized = query.trim().toLowerCase();
  let totalVisible = 0;
  document.querySelectorAll(".nav-section").forEach((section) => {
    let visibleCount = 0;
    section.querySelectorAll("li").forEach((item) => {
      const visible = !normalized || item.dataset.search.includes(normalized);
      item.hidden = !visible;
      if (visible) visibleCount += 1;
    });
    section.hidden = visibleCount === 0;
    totalVisible += visibleCount;
    if (normalized && visibleCount) section.open = true;
  });
  if (normalized && totalVisible === 0) elements.progress.textContent = "No lesson titles match that search.";
  else updateNavigationState();
}

function applyTheme(theme) {
  if (theme === "auto") document.documentElement.removeAttribute("data-theme");
  else document.documentElement.dataset.theme = theme;
  elements.theme.textContent = `Theme: ${theme[0].toUpperCase()}${theme.slice(1)}`;
  try {
    localStorage.setItem(THEME_KEY, theme);
  } catch {
    // Theme selection still applies for the current page.
  }
  void renderMermaidDiagrams();
}

function cycleTheme() {
  const current = localStorage.getItem(THEME_KEY) || "auto";
  const next = current === "auto" ? "light" : current === "light" ? "dark" : "auto";
  applyTheme(next);
}

function openMobileMenu() {
  elements.sidebar.classList.add("is-open");
  elements.sidebar.inert = false;
  elements.menu.setAttribute("aria-expanded", "true");
  elements.scrim.hidden = false;
}

function closeMobileMenu() {
  elements.sidebar.classList.remove("is-open");
  elements.menu.setAttribute("aria-expanded", "false");
  elements.scrim.hidden = true;
  elements.sidebar.inert = window.matchMedia("(max-width: 780px)").matches;
}

function toggleMobileMenu() {
  if (elements.sidebar.classList.contains("is-open")) closeMobileMenu();
  else openMobileMenu();
}

function syncMenuMode() {
  if (window.matchMedia("(max-width: 780px)").matches) {
    if (!elements.sidebar.classList.contains("is-open")) elements.sidebar.inert = true;
  } else {
    elements.sidebar.inert = false;
    elements.scrim.hidden = true;
    elements.sidebar.classList.remove("is-open");
    elements.menu.setAttribute("aria-expanded", "false");
  }
}

renderNavigation();
applyTheme(localStorage.getItem(THEME_KEY) || "auto");
elements.search.addEventListener("input", (event) => filterNavigation(event.target.value));
elements.theme.addEventListener("click", cycleTheme);
elements.menu.addEventListener("click", toggleMobileMenu);
elements.scrim.addEventListener("click", closeMobileMenu);
elements.content.addEventListener("click", handleContentClick);
document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") closeMobileMenu();
});
window.addEventListener("hashchange", loadLesson);
window.addEventListener("resize", syncMenuMode);
window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
  if ((localStorage.getItem(THEME_KEY) || "auto") === "auto") void renderMermaidDiagrams();
});
syncMenuMode();
loadLesson();
