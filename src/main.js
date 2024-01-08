const Tauri = window.__TAURI__.tauri;
const  invoke = Tauri.invoke;
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
    updateFileList();
})
async function updateFileList() {
    console.log(await invoke('get_files'));
    try {
        const files = await invoke('get_files');

        const filelist = document.getElementById("files");
        files.forEach((file) => {
            console.log(file)
            const listItem = document.createElement('li');
            listItem.textContent = file;
            listItem.addEventListener('click', () => {
                openEditor(file);
                console.log("added event listener to LI");
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

