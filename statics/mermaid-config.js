import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10.2.3/dist/mermaid.esm.min.mjs';

export function initializeMermaid() {
    mermaid.initialize({ startOnLoad: true });
}

export function renderMermaid() {
    mermaid.contentLoaded();
}