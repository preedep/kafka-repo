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
export async function renderMermaid(diagramText) {
    console.log("Rendering Mermaid diagram");

    // Example of using the render function
    const drawDiagram = async function () {
        const element = document.getElementById('mermaid-container-display');
        const {svg} = await mermaid.render('graphDiv', diagramText);
        element.innerHTML = svg;
    };

    await drawDiagram();
}