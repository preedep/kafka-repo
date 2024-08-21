//import marked from './marked.min.js';
export function renderMarked(markdownText) {
    const element = document.getElementById('ai-search-result-container');
    element.innerHTML = window.marked(markdownText); // Use window.marked
}