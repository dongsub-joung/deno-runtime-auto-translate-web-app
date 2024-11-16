function submitText() {
    // Get the input text
    const inputText = document.getElementById("inputText").value;

    // Check if there's input
    if (inputText.trim()) {
        // Display the input text below the form
        document.getElementById("output").innerHTML = `
            <h2>Your Submitted Text:</h2>
            <pre>${inputText}</pre>
        `;
    } else {
        // Alert if no text is entered
        alert("Please enter some text!");
    }
}

