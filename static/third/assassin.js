// Assassin.js does all the dirty work in the program.


// Ctrl + /.
let src_view_button = document.querySelector('button#src-button.status-bar-button');
document.addEventListener('keydown', event => {
    if (event.ctrlKey && event.code === "Slash") {
        document.querySelector('button#src-button.status-bar-button').click()
    }
});

// "Autofocus" src-input.
src_view_button.addEventListener('click', _ => {
    if (src_view_button.getAttribute("view_status") === true) {
        let src_input = document.querySelector('.src textarea.src-input');
        src_input.focus()
    }
});

// Export Clipboard.
let export_button = document.querySelector('button#export-button.status-bar-button');
new ClipboardJS(export_button);
