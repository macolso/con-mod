/* 
Backend Expectations:
The API should accept a POST request at the /api/moderation endpoint with a JSON body containing a url field.
The backend should return a JSON object in the following format:
json
Copy
Edit
{
    "results": [
        {
            "flagged": true,
            "categories": {
                "sexual": false,
                "hate": false,
                "harassment": true,
                "self-harm": false,
                "sexual/minors": false,
                "hate/threatening": false,
                "violence/graphic": false,
                "self-harm/intent": false,
                "self-harm/instructions": false,
                "harassment/threatening": true,
                "violence": true
            }
        }
    ]
}
*/
document.addEventListener('DOMContentLoaded', () => {
    // Function to handle moderation check
    function checkModeration() {
        console.log("here")
        const url = urlInput.value;
        if (!url) {
            alert("Please enter a website URL.");
            return;
        }

        // Fetch moderation result from API
        apiRequest(`${window.location.href}/api`, 'POST', url)
            .then(data => {
                const result = data.results[0];
                if (result.flagged) {
                    statusDiv.textContent = "⚠️ Content is flagged";
                    statusDiv.classList.add("flagged");
                } else {
                    statusDiv.textContent = "✅ Content is safe";
                    statusDiv.classList.add("not-flagged");
                }

                const flaggedCategories = Object.entries(result.categories)
                    .filter(([category, isFlagged]) => isFlagged)
                    .map(([category]) => category.replace(/\//g, ' → '));

                if (flaggedCategories.length > 0) {
                    categoriesDiv.innerHTML = "<h3>Flagged Categories:</h3>" +
                        flaggedCategories.map(cat => `<div class='category flagged-category'>⚠️ ${cat}</div>`).join('');
                } else {
                    categoriesDiv.innerHTML = "<h3>No specific flagged categories.</h3>";
                }
            })
            .catch(error => {
                console.error('Moderation check failed:', error);
                statusDiv.textContent = "❌ Error in moderation check";
                categoriesDiv.innerHTML = "";
            });
    }
    const urlInput = document.getElementById('urlInput');
    const statusDiv = document.getElementById('status');
    const categoriesDiv = document.getElementById('categories');

    // API request function
    const apiRequest = (url, method = 'GET', body = null) => {
        const options = {
            method: method,
            headers: {
                'Content-Type': 'text/plain'
            }
        };
        if (body) {
            options.body = body;
        }

        return fetch(url, options)
            .then(response => response.json())
            .catch(error => console.error('API request failed:', error));
    };

    // Event listener for checking moderation
    document.querySelector('img').addEventListener('click', checkModeration);
});