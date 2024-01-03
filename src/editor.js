const Tauri = window.__TAURI__.tauri;
const  invoke = Tauri.invoke;

document.getElementById("returnButton").addEventListener('click', () => {
    window.location.href = `index.html`
});
document.getElementById("saveButton").addEventListener('click', () => {
    let content = document.getElementById("editor").value;

        invoke("save_file", {fileContent: content}, {window: Tauri.window})
});
document.getElementById("deleteButton").addEventListener('click', () => {
    let content = document.getElementById("editor").value;
    invoke("delete_this_file",{fileName: document.getElementById("fileName").textContent})
    updateFileList();
    window.location.href = "index.html";
});
function getParams(name) {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(name);
}

function get_file(name) {
    const editor = document.getElementById("editor");
    const editorFileName = document.getElementById("fileName");
    let content = invoke("get_file_content", {fileName: name}).then((result) => {
        editor.textContent = result;
        editorFileName.textContent = name
    });

}
async function updateFileList() {
    try {
        const files = await invoke('get_files');

        const filelist = document.getElementById("filesList");
        filelist.innerHTML = "";
        files.forEach((file) => {
            const listItem = document.createElement('div');
            listItem.textContent = file;
            listItem.addEventListener('click', () => {
                openEditor(file);
            })
            listItem.classList.add("fileListItem")
            filelist.appendChild(listItem);
        })
    } catch (error) {
        console.log(error);
    }
}
function openEditor(filename) {
    window.location.href = `editor.html?file=${encodeURIComponent(filename)}`
}

addEventListener("DOMContentLoaded", () => {
    get_file(getParams("file"));
    updateFileList();
})