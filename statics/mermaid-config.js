import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10.2.3/dist/mermaid.esm.min.mjs';
export function initializeMermaid() {
    console.log("Initializing Mermaid");
    mermaid.initialize(
        {
            startOnLoad: false,
            securityLevel: 'loose',
            theme: 'default',
        }
    );
}
export function renderMermaid(diagramText) {
    console.log("Rendering Mermaid diagram");

    const container = document.getElementById('mermaid-container');
    if (!container) {
        console.error('Mermaid container not found');
        return;
    }
    container.innerHTML = ''; // Clear previous content
    try {
        console.log(diagramText);
        mermaid.mermaidAPI.render('mermaid-container-1',
            diagramText,
            (svgCode) => {
            container.innerHTML = svgCode;
            console.log("Successfully rendered chart mermaid-svg.");
        }, container);

    } catch (e) {
        console.error(`FAILED to render chart mermaid-svg: ${e}`);
    }
}