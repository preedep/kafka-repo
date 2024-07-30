import { initializeMermaid, renderMermaid } from './mermaid-config.js';
import {  renderTable} from './table.js';
import { filterFunction, handleKeyDown , selectItem} from './searchable-dropdown.js';

function downloadSVG() {
    const svg = document.getElementById('mermaid-container');
    const serializer = new XMLSerializer();
    const svgString = serializer.serializeToString(svg);

    const blob = new Blob([svgString], {type: 'image/svg+xml'});
    const url = URL.createObjectURL(blob);

    downloadFile(url, 'image.svg');
}

function downloadCSV() {
    const headers = Array.from(document.querySelectorAll('#table-head th')).map(th => th.innerText);
    const rows = Array.from(document.querySelectorAll('#table-body tr')).map(tr =>
        Array.from(tr.querySelectorAll('td')).map(td => td.innerText)
    );

    let csvContent = 'data:text/csv;charset=utf-8,';
    csvContent += headers.join(',') + '\n';
    rows.forEach(row => {
        csvContent += row.join(',') + '\n';
    });

    const encodedUri = encodeURI(csvContent);

    downloadFile(encodedUri, 'data.csv');
}

function downloadFile(blobType, fileName) {
    const link = document.createElement('a');
    link.setAttribute('href', blobType);
    link.setAttribute('download', fileName);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}

function button_download_svg_handler(){
    const button = document.getElementById('downloadSvgButton');
    button.addEventListener('click', function() {
        downloadSVG();
    });
}
function button_download_csv_handler(){
    const button = document.getElementById('downloadCsvButton');
    button.addEventListener('click', function() {
        downloadCSV();
    });
}
function load_dropdown_owner_of_topics() {
    // API endpoint
    const apiEndpoint = '/api/v1/apps';

    // Fetch data from the API
    fetch(apiEndpoint)
        .then(response => response.json())
        .then(data => {
            const dropdown = document.getElementById('dropdown-owner-topic');
            bind_data_for_option(data, dropdown);
            console.log(data);

        })
        .catch(error => console.error('Error fetching data:', error));
}

function load_dropdown_topics(app_owner_name) {
    // API endpoint
    const apiEndpoint = `/api/v1/apps/${app_owner_name}/topics`;
    // Fetch data from the API
    fetch(apiEndpoint)
        .then(response => response.json())
        .then(data => {
            const dropdown = document.getElementById('dropdown-topic-name');
            dropdown.innerHTML = '<option value="0">Select an Topic Name</option>';
            bind_data_for_option(data, dropdown);

            // Searchable dropdown
            const searchable_dropdown = document.getElementById('dropdown');
            searchable_dropdown.innerHTML = ''
            for (let i = 0; i < data.data.length; i++) {
                const item = data.data[i];
                const div = document.createElement('div');
                div.textContent = item;
                div.addEventListener('click', function() {
                    selectItem(this);
                });
                searchable_dropdown.appendChild(div);
            }
            /////

            console.log(data);

        })
        .catch(error => console.error('Error fetching data:', error));
}
function detect_change_topic_name() {
    console.log("detect_change_topic_name");
    const dropdown = document.getElementById('dropdown-topic-name');
    dropdown.addEventListener('change', function() {
       if (this.value === '0') {
           const search_dropdown_topic = document.getElementById('dropdown-topic-name-input');
           search_dropdown_topic.value = ''// clear search input
       }
    });
}

function detect_change_owner_of_topics() {
    console.log("detect_change_owner_of_topics");

    const dropdown = document.getElementById('dropdown-owner-topic');
    const dropdownLabel = dropdown.previousElementSibling;

    if (!dropdown){
        console.log("Not found dropdown-owner-topic");
        return
    }

    dropdown.addEventListener('change', function() {
        const app_owner_name = this.value;
        console.log("Owner Topic: ", app_owner_name);

        const dropdown_topic = document.getElementById('dropdown-topic-name');

        const search_dropdown_topic = document.getElementById('dropdown-topic-name-input');
        search_dropdown_topic.value = ''// clear search input

        if (app_owner_name !== '0') {
            console.log("Select Topic Owner");
            // Show topic name under Topic owner
            dropdown_topic.innerText = '';
            // Show the dropdown label
            dropdownLabel.style.display = 'block';
            //dropdownLabel.style.paddingTop = '10px';
            // Show the dropdown
            dropdown_topic.style.display = 'block';

            search_dropdown_topic.style.display = 'block';
            //dropdown_topic.style.paddingTop = '10px';
            // Load the dropdown for the selected owner
            load_dropdown_topics(app_owner_name);
        }else{
            console.log("Not select Topic Owner");

            dropdown_topic.style.display = 'none';

            search_dropdown_topic.style.display = 'none';
            //dropdownLabel.style.display = 'none';
        }
    });
}
// Loop through the data and create option elements
function bind_data_for_option(data, dropdown) {
    const items = data.data;
    for (let i = 0; i < items.length; i++) {
        const item = items[i];
        const option = document.createElement('option');
        option.value = item;
        option.textContent = item;
        dropdown.appendChild(option);
    }
}

function load_dropdown_app_consumer() {
    // API endpoint
    const apiEndpoint = '/api/v1/consumers';

    // Fetch data from the API
    fetch(apiEndpoint)
        .then(response => response.json())
        .then(data => {
            const dropdown = document.getElementById('dropdown-consumer-app');
            bind_data_for_option(data, dropdown);
            console.log(data);

        })
        .catch(error => console.error('Error fetching data:', error));

}
function button_search_handler(){
    const button = document.getElementById('searchButton');
    button.addEventListener('click', function() {
        // Replace with your API URL
        const apiEndpoint = '/api/v1/search';

        const dropdown_owner_topic = document.getElementById('dropdown-owner-topic');
        const dropdown_topic_name = document.getElementById('dropdown-topic-name');
        const dropdown_consumer_app = document.getElementById('dropdown-consumer-app');

        const owner_topic = dropdown_owner_topic.value;
        const topic_name = dropdown_topic_name.value;
        const consumer_app = dropdown_consumer_app.value;

        console.log("Owner Topic: ", owner_topic);
        console.log("Topic Name: ", topic_name);
        console.log("Consumer App: ", consumer_app);


        let json_data_req = {
        };
        if (owner_topic !== '0') {
            json_data_req.app_owner = owner_topic;
        }
        if ((topic_name !== '0')&&(dropdown_topic_name.style.display !== 'none')) {
            json_data_req.topic_name = topic_name;
        }
        if (consumer_app !== '0') {
            json_data_req.consumer_app = consumer_app;
        }

        fetch(apiEndpoint, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(json_data_req),
        })
            .then(response => response.json())
            .then(data => {
                console.log('Success:', data);
                //const result = document.getElementById('result');
                //result.innerHTML = JSON.stringify(data, null, 2);
                if (data.data.length === 0) {
                    alert("No data found");

                    let table = document.getElementById('table-container');
                    table.style.display = 'none';

                    return;
                }

                let table = document.getElementById('table-container');
                table.style.display = 'block';

                let mermaid = document.getElementById('mermaid-container');
                mermaid.style.display = 'none';

                renderTable(data.data);
            })
            .catch((error) => {
                console.error('Error:', error);
            });

    });
}


function button_render_handler(){
    const button = document.getElementById('renderButton');


    button.addEventListener('click', function() {
        // Replace with your API URL
        const apiEndpoint = '/api/v1/render';

        const dropdown_owner_topic = document.getElementById('dropdown-owner-topic');
        const dropdown_topic_name = document.getElementById('dropdown-topic-name');
        const dropdown_consumer_app = document.getElementById('dropdown-consumer-app');

        const owner_topic = dropdown_owner_topic.value;
        const topic_name = dropdown_topic_name.value;
        const consumer_app = dropdown_consumer_app.value;

        console.log("Owner Topic: ", owner_topic);
        console.log("Topic Name: ", topic_name);
        console.log("Consumer App: ", consumer_app);


        let json_data_req = {
        };
        if (owner_topic !== '0') {
            json_data_req.app_owner = owner_topic;
        }
        if ((topic_name !== '0') && (dropdown_topic_name.style.display !== 'none')) {
            json_data_req.topic_name = topic_name;
        }
        if (consumer_app !== '0') {
            json_data_req.consumer_app = consumer_app;
        }

        fetch(apiEndpoint, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(json_data_req),
        })
            .then(response => response.text())
            .then(async data => {
                console.log('Success:', data);
                console.log("renderMermaid with data");

                if (data.length === 0) {
                    alert("No data found");

                    let mermaid = document.getElementById('mermaid-container');
                    mermaid.style.display = 'none';

                    return;
                }

                let table = document.getElementById('table-container');
                table.style.display = 'none';

                let mermaid = document.getElementById('mermaid-container');
                mermaid.style.display = 'block';


                initializeMermaid();
                await renderMermaid(data);

            })
            .catch((error) => {
                console.error('Error:', error);
            });

    });
}






// Load the dropdown when the DOM is ready

function search_able_dropdown_topic_name_handler() {
    document.getElementById("dropdown-topic-name-input").addEventListener("input", filterFunction);
    document.getElementById("dropdown-topic-name-input").addEventListener("keydown", handleKeyDown);
}

document.addEventListener('DOMContentLoaded', function() {
    console.log("initializeMermaid");
    initializeMermaid();

    load_dropdown_owner_of_topics();
    load_dropdown_app_consumer();
    detect_change_owner_of_topics();
    detect_change_topic_name();
    button_search_handler();
    button_render_handler();
    button_download_csv_handler();
    button_download_svg_handler();

    search_able_dropdown_topic_name_handler()
});