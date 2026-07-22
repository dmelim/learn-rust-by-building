# Course reader

The repository root contains a static HTML reader for the Markdown course. The
Markdown files remain the source of truth.

From Git Bash, start a local server:

```console
bash scripts/serve-course.sh
```

Then open <http://localhost:8000>. You can also use the local-server or live
preview feature in your code editor.

Opening `index.html` directly with a `file://` URL will not work in browsers
that block local `fetch` requests. Serving the repository over HTTP avoids that
browser restriction.

## Files

- `index.html`: accessible application shell
- `site/course.css`: responsive light and dark visual system
- `site/course.js`: course manifest, Markdown renderer, routing, and progress
- `site/vendor/mermaid`: pinned Mermaid 11.16.0 browser renderer and license

Fenced `mermaid` blocks render as responsive, theme-aware diagrams. Their source
remains available from the **View diagram source** disclosure beneath each
diagram. Mermaid is served locally, so diagrams do not require a CDN connection.

No generated HTML copies are committed. Updating a Markdown lesson updates the
reader the next time it is loaded.
