import { initializeMermaid, renderMermaid } from './mermaid-config.js';
import {  renderTable} from './table.js';
import { filterFunction, handleKeyDown , selectItem} from './searchable-dropdown.js';
import { checkTokenValidity } from './token-handler.js';

function load_filter_table() {
    const input = document.getElementById('table-result-search-input');
    input.addEventListener('keyup', filterTable);
}
function filterTable() {
    const input = document.getElementById('table-result-search-input');
    const filter = input.value.toLowerCase();
    const table = document.getElementById('data-table');
    const tbody = table.getElementsByTagName('tbody')[0];
    const rows = tbody.getElementsByTagName('tr');

    for (let i = 0; i < rows.length; i++) {
        const cells = rows[i].getElementsByTagName('td');
        let rowContainsFilter = false;

        for (let j = 0; j < cells.length; j++) {
            const cell = cells[j];
            const text = cell.innerText.toLowerCase();
            const originalText = cell.innerText;

            // Remove existing highlights
            cell.innerHTML = originalText;

            if (text.includes(filter) && filter !== '') {
                rowContainsFilter = true;
                const startIndex = text.indexOf(filter);
                const endIndex = startIndex + filter.length;
                const highlightedText = originalText.substring(startIndex, endIndex);
                const highlightedHTML = originalText.substring(0, startIndex) +
                    '<span class="highlight">' + highlightedText + '</span>' +
                    originalText.substring(endIndex);
                cell.innerHTML = highlightedHTML;
            }
        }

        rows[i].style.display = rowContainsFilter ? '' : 'none';
    }
}

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


        fetchDataWithAccessToken(apiEndpoint,'GET',null).then(response => response.json())
        .then(data => {
            const dropdown = document.getElementById('dropdown-owner-topic');
            bind_data_for_option(data, dropdown);
            console.log(data);

        })
        .catch(error => {
            console.error('Error fetching data:', error)
            //alert(error);
        });
}

function load_dropdown_topics(app_owner_name) {
    // API endpoint
    const apiEndpoint = `/api/v1/apps/${app_owner_name}/topics`;

    fetchDataWithAccessToken(apiEndpoint,'GET',null).then(response => response.json())
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
    fetchDataWithAccessToken(apiEndpoint,'GET',null)
        .then(response => response.json())
        .then(data => {
            const dropdown = document.getElementById('dropdown-consumer-app');
            bind_data_for_option(data, dropdown);
            console.log(data);

        })
        .catch(error => console.error('Error fetching data:', error));

}

function get_search_data_req() {

    const ai_search_input = document.getElementById('ai-search-input');
    const dropdown_owner_topic = document.getElementById('dropdown-owner-topic');
    const dropdown_topic_name = document.getElementById('dropdown-topic-name');
    const dropdown_consumer_app = document.getElementById('dropdown-consumer-app');

    const owner_topic = dropdown_owner_topic.value.trim();
    const topic_name = dropdown_topic_name.value.trim();
    const consumer_app = dropdown_consumer_app.value.trim();
    const ai_search_value = ai_search_input.value.trim();

    console.log("Owner Topic: ", owner_topic);
    console.log("Topic Name: ", topic_name);
    console.log("Consumer App: ", consumer_app);
    console.log("AI Search: ", ai_search_value);

    let json_data_req = {};
    if (owner_topic !== '0') {
        json_data_req.app_owner = owner_topic;
    }
    if ((topic_name !== '0') && (dropdown_topic_name.style.display !== 'none')) {
        json_data_req.topic_name = topic_name;
    }
    if (consumer_app !== '0') {
        json_data_req.consumer_app = consumer_app;
    }
    if (ai_search_input.value.trim() !== '') {
        json_data_req.ai_search_query = ai_search_value;
    }

    return json_data_req;
}

function button_ai_search_handler(){
    const button = document.getElementById('ai_searchButton');


    button.addEventListener('click', function() {
        const input = document.getElementById('ai-search-input');

        if (input.value === '') {
            alert('Please enter a search query');
            return;
        }

        let json_data_req = get_search_data_req();

        document.getElementById('table-container').style.display = 'none';
        document.getElementById('mermaid-container').style.display = 'none';
        // Show the loading screen
        document.getElementById('ai-search-result-loading').style.display = 'block';
        document.getElementById('ai-search-result-loading').style.display = 'flex';

        // Replace with your API URL
        const apiEndpoint = '/api/v1/ai_search';
        // Fetch data from the API

        fetchDataWithAccessToken(apiEndpoint,'POST',json_data_req)
            .then(response => {
                // Try to extract the error message from the response
                if (!response.ok) {
                    return response.json().then(errData => {
                        // Throw an error with the server's error message
                        throw new Error(errData.error || `HTTP error! status: ${response.status}`);
                    });
                }
                return response.json();
            })
            .then(data => {
                console.log(data);
                // Hide the loading screen
                document.getElementById('ai-search-result-loading').style.display = 'none';
                // Show the search results
                let ai_result = data.data;
                let all_content = '';
                for (let i = 0; i < ai_result.choices.length;i++){
                    let choice = ai_result.choices[i];
                    let message = choice.message;
                    let content = message.content;
                    all_content = all_content + content;
                }
                console.log("All content: ", all_content);
                document.getElementById('ai-search-result-container').style.display = 'block';
                // Step 1: Split the string by newlines
                let lines = all_content.split('\n');
                // Step 2: Wrap each line in a <p> tag
                let paragraphs = lines.map(line => `<p>${line}</p>`);
                // Step 3: Join the array into a single string
                let htmlText = paragraphs.join('');
                document.getElementById('ai-search-result-container').innerHTML = htmlText;

            })
            .catch((error) => {
                console.error('Error:', error);
                // Hide the loading screen
                document.getElementById('ai-search-result-loading').style.display = 'none';
            });
    });
}

// Fetch data from the API with an access token
function fetchDataWithAccessToken(apiEndpoint, method, body) {
    // Fetch data from the API
    let accessToken = localStorage.getItem('token');
    let params = {
        method: method,
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${accessToken}`,
        }
    }
    if (method === 'POST') {
        if (body !== null) {
            params.body = JSON.stringify(body);
        }
    }

    return fetch(apiEndpoint, params)
        .then(response => {
            if (!response.ok) {
                return response.json().then(errData => {
                    throw new Error(errData.error || `HTTP error! status: ${response.status}`);
                });
            }
            return response.json();
        })
        .catch(error => {
            console.error('Error:', error);
            throw error;
        });
}

function button_search_handler(){
    const button = document.getElementById('searchButton');
    button.addEventListener('click', function() {
        // Replace with your API URL
        const apiEndpoint = '/api/v1/search';

        let json_data_req = get_search_data_req();

        fetchDataWithAccessToken(apiEndpoint,'POST',json_data_req)
            .then(response => {
                // Try to extract the error message from the response
                if (!response.ok) {
                    return response.json().then(errData => {
                        // Throw an error with the server's error message
                        throw new Error(errData.error || `HTTP error! status: ${response.status}`);
                    });
                }
                return response.json();
            })
            .then(data => {
                console.log('Success:', data);
                //const result = document.getElementById('result');
                //result.innerHTML = JSON.stringify(data, null, 2);
                if (data.data.length === 0) {
                    document.getElementById('table-container').style.display = 'none';
                    document.getElementById('ai-search-result-container').style.display = 'none';
                    document.getElementById('mermaid-container').style.display = 'none';

                    alert("No data found");
                    return;
                }
                document.getElementById('table-container').style.display = 'block';
                document.getElementById('ai-search-result-container').style.display = 'none';
                document.getElementById('mermaid-container').style.display = 'none';


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

            // render by api
            // Replace with your API URL
            const apiEndpoint = '/api/v1/render';

            let json_data_req = get_search_data_req();

            const table_input_search = document.getElementById('table-result-search-input');
            console.log("input ",table_input_search.value);
            if (table_input_search.value !== '') {
                console.log("Search all text: ", table_input_search.value);
                json_data_req.search_all_text = table_input_search.value;
            }

            fetchDataWithAccessToken(apiEndpoint,'POST',json_data_req)
                .then(response => response.text())
                .then(async data => {
                    console.log('Success:', data);
                    console.log("renderMermaid with data");

                    if (data.length === 0) {
                        document.getElementById('mermaid-container').style.display = 'none';
                        document.getElementById('table-container').style.display = 'none';
                        document.getElementById('ai-search-result-container').style.display = 'none';
                        alert("No data found");
                        return;
                    }
                    document.getElementById('table-container').style.display = 'none';
                    document.getElementById('ai-search-result-container').style.display = 'none';
                    document.getElementById('mermaid-container').style.display = 'block';

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
    if (!checkTokenValidity()) {
        window.location.href = 'login.html'; // Redirect to login if token is invalid or expired
    }

    console.log("initializeMermaid");
    initializeMermaid();

    // load drop down list
    load_dropdown_owner_of_topics();
    load_dropdown_app_consumer();
    // detect change dropdown
    detect_change_owner_of_topics();
    detect_change_topic_name();
    // button handler
    button_search_handler();
    button_render_handler();
    button_ai_search_handler();
    button_download_csv_handler();
    button_download_svg_handler();

    // search all text in table search result
    load_filter_table();
    //search in dropdown list
    search_able_dropdown_topic_name_handler()
});