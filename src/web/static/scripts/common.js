// Here is the common methods that are used in the application

// Method for loading the exchanges by get request from /api/exchange/exchanges route
async function loadExchanges() {
    const exchangeSelect = document.querySelector('select[name="exchange"]');
    try {
        const response = await fetch("/api/exchange/exchanges");
        const data = await response.json();
        exchangeSelect.innerHTML = '<option selected disabled value="">Select exchange</option>';
        data.forEach((exchange) => {
            const option = document.createElement("option");
            option.value = exchange;
            option.textContent = exchange;
            exchangeSelect.appendChild(option);
        });
    } catch (error) {
        console.error('Error loading exchanges:', error);
    }
}
