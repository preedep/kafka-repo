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

// Load the dropdown when the DOM is ready

document.addEventListener('DOMContentLoaded', function() {
    load_dropdown_owner_of_topics();
    load_dropdown_app_consumer();
    detect_change_owner_of_topics();
});