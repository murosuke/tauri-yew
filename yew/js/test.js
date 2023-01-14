export async function openFilePicker() {
    console.log("js called!");
    return await window.showOpenFilePicker();
}