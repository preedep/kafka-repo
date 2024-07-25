document.addEventListener('DOMContentLoaded', function() {
    // API endpoint
    const apiEndpoint = '/api/v1/apps';

    // Fetch data from the API
    fetch(apiEndpoint)
        .then(response => response.json())
        .then(data => {
            const dropdown = document.getElementById('dropdown');
            // Loop through the data and create option elements
            const items = data.data;
            for (let i = 0; i < items.length; i++) {
                const item = items[i];
                const option = document.createElement('option');
                option.value = item;
                option.textContent = item;
                dropdown.appendChild(option);
            }
            console.log(data);

        })
        .catch(error => console.error('Error fetching data:', error));
});