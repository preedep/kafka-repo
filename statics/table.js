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

    // Reassign row numbers after sorting
    rows.forEach((row, index) => {
        row.getElementsByTagName("td")[0].textContent = index + 1 + '';
    });
}

export function renderTable(data) {
    const tableHead = document.getElementById('table-head');
    const tableBody = document.getElementById('table-body');

    // Clear existing table content
    tableHead.innerHTML = '';
    tableBody.innerHTML = '';

    // Create the first column header for the row numbers
    const thNumber = document.createElement('th');
    thNumber.textContent = '#';
    tableHead.appendChild(thNumber);

    // Get the keys from the first object to create the table headers
    const headers = Object.keys(data[0]);

    // Create table headers for data
    headers.forEach((header, index) => {
        const th = document.createElement('th');
        th.textContent = header.charAt(0).toUpperCase() + header.slice(1);
        th.addEventListener('click', () => sortTable(index + 1)); // Adjust index for sorting
        tableHead.appendChild(th);
    });

    // Create table rows
    data.forEach((item, rowIndex) => {
        const tr = document.createElement('tr');

        // Create the first cell with the row number
        const tdNumber = document.createElement('td');
        tdNumber.textContent = rowIndex + 1; // Row numbers start from 1
        tr.appendChild(tdNumber);

        // Create cells for data
        headers.forEach(header => {
            const td = document.createElement('td');
            td.textContent = item[header];
            tr.appendChild(td);
        });

        tableBody.appendChild(tr);
    });
}