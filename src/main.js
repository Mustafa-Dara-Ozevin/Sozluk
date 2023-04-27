const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let meaningsEl;

async function search() {
  
  const meanings = await invoke("search", { query: greetInputEl.value })
  let meaning = "";
  meaningsEl.textContent = '';
  if (meanings.length === 0){
    
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

  document
    .querySelector("#search-button")
    .addEventListener("click", () => search());

  
  document
  .querySelector("#search-input")
  .addEventListener("keyup", () => search())
  
});
