//import marked from './marked.min.js';
export function renderMarked(markdownText) {
    const element = document.getElementById('ai-search-result-container');
    if (markdownText.length === 0) {
        element.style.display = 'none';
    }else{
        element.style.display = 'block';
    }
    element.innerHTML = window.marked.parse(markdownText); // Use window.marked
}