import("./index.js")
    .catch(e => console.error("Error importing `index.js`:", e));

//const index = import("./index.js");
//index.then(() => {
//    console.log("WebAssembly module loaded...");
//});