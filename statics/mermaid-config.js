import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10.2.3/dist/mermaid.esm.min.mjs';
export function initializeMermaid() {
    console.log("Initializing Mermaid");
    mermaid.initialize({ startOnLoad: false });
}
export function renderMermaid(diagramText) {
    const container = document.getElementById('mermaid-container');
    let mermaidAPI = mermaid.mermaidAPI;


    container.innerHTML = ''; // Clear previous content
/*
    var insertSvg = function (svgCode, bindFunctions) {
        container.innerHTML = svgCode;
    };
    mermaidAPI.render('mermaid-chart', diagramText, insertSvg)
        .then(() => {
            console.log(`Successfully rendered chart mermaid-svg.`);
        })
        .catch((e) => {
            console.log(`FAILED to render chart mermaid-svg: ${e}`);
        });
 */



    mermaidAPI.render('mermaid-diagram', diagramText, (svgCode) => {
        container.innerHTML = svgCode;
    }, container);
}