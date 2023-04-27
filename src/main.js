const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let meaningsEl;
let titleEl;

async function search() {
  let text =  greetInputEl.value
  const meanings = await invoke("search", { query: text })
  titleEl.textContent = text;
  let meaning = "";
  meaningsEl.textContent = '';
  if (meanings.length === 0){
    return
  }else{
    for (meaning of meanings){
      const li = document.createElement("li");
      li.textContent = meaning;
      meaningsEl.appendChild(li);
    }

  }

  
  
}



window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#search-input");
  meaningsEl = document.querySelector("#meanings");
  titleEl = document.querySelector("#title");

  document
    .querySelector("#search-button")
    .addEventListener("click", () => search());

  
  document
  .querySelector("#search-input")
  .addEventListener("keyup", () => search())
  
});