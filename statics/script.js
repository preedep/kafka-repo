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
});