const Tauri = window.__TAURI__.tauri;
const  invoke = Tauri.invoke;
/*import { useKeyPress } from '/ahooks';
useKeyPress(['ctrl.s'], async () => {
    await autoSaveContent()
})*/


document.getElementById("returnButton").addEventListener('click', () => {
    window.location.href = `index.html`
});
document.getElementById("addHeaderButton").addEventListener('click', () => {
    let content = document.getElementById("editor");
    console.log("reached part 1")
    content.value = content.value + "\n[=========================================================================================]\n" +
        "                                       NEXT SUBJECT\n" +
        "[=========================================================================================]\n"
    console.log("should have added header")
})
document.getElementById("addPointButton").addEventListener('click', () => {
    let content = document.getElementById("editor");
    console.log("reached part 1")
    content.value = content.value + " - \n"
    console.log("should have added point")
})

document.getElementById("saveButton").addEventListener('click', async () => {
    let button = document.getElementById("saveButton");
    button.id = "saveButtonClicked";
    console.log(button.id)
    await new Promise(r => setTimeout(r, 2000));
    let content = document.getElementById("editor").value;
    button.id = "saveButton";

    console.log(button.id)
    invoke("save_file", {fileContent: content}, {window: Tauri.window});

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
async function autoSaveContent() {
    let button = document.getElementById("saveButton");
    button.id = "saveButtonClicked";
    console.log(button.id)
    await new Promise(r => setTimeout(r, 2000));
    let content = document.getElementById("editor").value;
    button.id = "saveButton";

    console.log(button.id)
    invoke("save_file", {fileContent: content}, {window: Tauri.window});
    await new Promise(r => setTimeout(r, 150000));
    await autoSaveContent();
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

        const filelist = document.getElementById("filesListContent");
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

addEventListener("DOMContentLoaded", async () => {
    get_file(getParams("file"));
    await updateFileList();
    await autoSaveContent()
})