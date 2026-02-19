/**
 * Fetches CSV data from the endpoint and updates the DOM.
 */
function updateTemperature() {
    const endpoint = 'http://blog.local/proxy/influxdb/freezer';
    const elements = document.getElementsByClassName('freezer-temperature');

    // Start the fetch request
    fetch(endpoint)
        .then(response => {
            if (!response.ok) throw new Error('Network error');
            return response.text(); // Get response as plain text (CSV)
        })
        .then(jsonData => {
            // Parsing Logic: 
            // 1. Split by newline to get rows
            // 2. Split first row by comma to get columns
            const temperature_degF = JSON.parse(jsonData)[0].Temperature

            // Update the UI
            if (elements) {
                Array.from(elements).forEach(element => {
                    element.textContent = temperature_degF
                });
            }
        })
        .catch(error => {
            console.error('Fetch failed:', error);
            if (elements) {
                Array.from(elements).forEach(element => {
                    element.textContent = "--";
                });
            }
        });
}

// --- Initialization ---

// 1. Run once immediately so the page isn't blank for the first 5 seconds
updateTemperature();

setInterval(updateTemperature, 3000);
