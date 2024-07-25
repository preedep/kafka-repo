import { initializeMermaid, renderMermaid } from './mermaid-config.js';

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

            console.log(data);

        })
        .catch(error => console.error('Error fetching data:', error));
}

function detect_change_owner_of_topics() {
    const dropdown = document.getElementById('dropdown-owner-topic');
    const dropdownLabel = dropdown.previousElementSibling;
    dropdown.addEventListener('change', function() {
        const app_owner_name = this.value;
        const dropdown_topic = document.getElementById('dropdown-topic-name');
        if (app_owner_name !== '0') {
            dropdown_topic.innerText = '';
            // Show the dropdown
            dropdown_topic.style.display = 'block';
            dropdown_topic.style.paddingTop = '10px';
            // Show the dropdown label
            dropdownLabel.style.display = 'block';
            dropdownLabel.style.paddingTop = '10px';
            // Load the dropdown for the selected owner
            load_dropdown_topics(app_owner_name);
        }else{
            dropdown_topic.style.display = 'none';
            dropdownLabel.style.display = 'none';
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
        if (topic_name !== '0') {
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
        if (topic_name !== '0') {
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
            .then(data => {
                console.log('Success:', data);
                console.log("renderMermaid with data");
                //renderMermaid(data);

                // Open a new window
                const newWindow = window.open('',
                    '_blank',
                    'width=600,height=400');

                // Write the HTML content to the new window's document
                newWindow.document.open();
                newWindow.document.write(data);
                newWindow.document.close();

            })
            .catch((error) => {
                console.error('Error:', error);
            });

    });
}
// Load the dropdown when the DOM is ready

document.addEventListener('DOMContentLoaded', function() {
    console.log("initializeMermaid");
    initializeMermaid();

    load_dropdown_owner_of_topics();
    load_dropdown_app_consumer();
    detect_change_owner_of_topics();
    button_search_handler();
    button_render_handler();
});