export function sortTable(columnIndex) {
    const table = document.getElementById("data-table");
    const tbody = table.getElementsByTagName("tbody")[0];
    const rows = Array.from(tbody.getElementsByTagName("tr"));
    const isAscending = table.getAttribute("data-sort-asc") === "true";

    rows.sort((a, b) => {
        const cellA = a.getElementsByTagName("td")[columnIndex].innerText.toLowerCase();
        const cellB = b.getElementsByTagName("td")[columnIndex].innerText.toLowerCase();

        if (cellA < cellB) {
            return isAscending ? -1 : 1;
        }
        if (cellA > cellB) {
            return isAscending ? 1 : -1;
        }
        return 0;
    });

    // Append sorted rows to tbody
    rows.forEach(row => tbody.appendChild(row));

    // Toggle sort direction
    table.setAttribute("data-sort-asc", !isAscending);
}


export function renderTable(data) {
    const tableHead = document.getElementById('table-head');
    const tableBody = document.getElementById('table-body');

    // Clear existing table content
    tableHead.innerHTML = '';
    tableBody.innerHTML = '';

    // Get the keys from the first object to create the table headers
    const headers = Object.keys(data[0]);

    // Create table headers
    headers.forEach((header, index) => {
        const th = document.createElement('th');
        th.textContent = header.charAt(0).toUpperCase() + header.slice(1);
        th.setAttribute('onclick', `sortTable(${index})`);
        tableHead.appendChild(th);
    });


    // Create table rows
    data.forEach(item => {
        const tr = document.createElement('tr');
        headers.forEach(header => {
            const td = document.createElement('td');
            td.textContent = item[header];
            tr.appendChild(td);
        });
        tableBody.appendChild(tr);
    });
}