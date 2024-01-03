const Tauri = window.__TAURI__.tauri;
const  invoke = Tauri.invoke;
const { tauri } = require('tauri/api/tauri');
const NewFileButton = document.getElementById("NewFileButton");
console.log("loaded js")
addEventListener('DOMContentLoaded', () => {
    updateFileList();
    console.log("doc:", document.getElementById("NewFileButton"));

})
NewFileButton.addEventListener('click', () => {
    const filename = document.getElementById("filenameinput").value;
    console.log(filename);
    invoke('create_file',{fileName: filename});
})
async function updateFileList() {
    try {
        const files = await invoke('get_files');

        const filelist = document.getElementById("files");
        files.forEach((file) => {
            const listItem = document.createElement('li');
            listItem.textContent = file;
            listItem.addEventListener('click', () => {
                openEditor(file);
            })
            filelist.appendChild(listItem);
        })
    } catch (error) {
        console.log(error);
    }
}
function openEditor(filename) {
    window.location.href = `editor.html?file=${encodeURIComponent(filename)}`
}

