let currentFocus = -1;


export function filterFunction() {
    let input, filter, div, items, i, txtValue;
    input = document.getElementById("dropdown-topic-name-input");
    filter = input.value.toUpperCase();
    div = document.getElementById("dropdown");
    items = div.getElementsByTagName("div");

    // Loop through all dropdown items, and hide those who don't match the search query
    for (i = 0; i < items.length; i++) {
        txtValue = items[i].textContent || items[i].innerText;
        if (txtValue.toUpperCase().indexOf(filter) > -1) {
            items[i].style.display = "";
            items[i].innerHTML = highlightMatch(txtValue, filter);
        } else {
            items[i].style.display = "none";
        }
    }

    // Show dropdown if there is any matching item
    if (filter === "" || Array.from(items).some(item => item.style.display === "")) {
        showDropdown();
    } else {
        closeDropdown();
    }
}

function highlightMatch(text, filter) {
    const startIndex = text.toUpperCase().indexOf(filter);
    if (startIndex >= 0) {
        const endIndex = startIndex + filter.length;
        return text.substring(0, startIndex) +
            "<span class='highlight'>" + text.substring(startIndex, endIndex) + "</span>" +
            text.substring(endIndex);
    }
    return text;
}

export function selectItem(element) {
    const input = document.getElementById("dropdown-topic-name-input");
    input.value = element.textContent || element.innerText;
    closeDropdown();
    updateSelectElement(input.value);
}

function showDropdown() {
    const div = document.getElementById("dropdown");
    div.classList.add("show");
}

function closeDropdown() {
    const div = document.getElementById("dropdown");
    div.classList.remove("show");
    currentFocus = -1; // Reset current focus
}

export function handleKeyDown(event) {
    const div = document.getElementById("dropdown");
    const items = Array.from(div.getElementsByTagName("div")).filter(item => item.style.display !== "none");

    if (event.keyCode === 40) { // Down arrow key
        currentFocus++;
        addActive(items);
    } else if (event.keyCode === 38) { // Up arrow key
        currentFocus--;
        addActive(items);
    } else if (event.keyCode === 13) { // Enter key
        event.preventDefault();
        if (currentFocus > -1) {
            if (items[currentFocus]) {
                selectItem(items[currentFocus]);
                closeDropdown(); // Close dropdown after selection
            }
        }
    } else if (event.keyCode === 27) { // Escape key
        closeDropdown();
    }
}

function addActive(items) {
    if (!items) return false;
    removeActive(items);
    if (currentFocus >= items.length) currentFocus = 0;
    if (currentFocus < 0) currentFocus = items.length - 1;
    items[currentFocus].classList.add("selected");
    items[currentFocus].scrollIntoView({ block: "nearest" });
}

function removeActive(items) {
    for (let i = 0; i < items.length; i++) {
        items[i].classList.remove("selected");
    }
}

function updateSelectElement(value) {
    const select = document.getElementById("dropdown-topic-name");
    for (let i = 0; i < select.options.length; i++) {
        if (select.options[i].text === value) {
            select.selectedIndex = i;
            break;
        }
    }
}

// Hide dropdown if clicked outside
document.addEventListener('click', function (event) {
    const isClickInside = document.querySelector('.dropdown-container').contains(event.target);

    if (!isClickInside) {
        closeDropdown();
    }
});