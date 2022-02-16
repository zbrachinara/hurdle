import * as hurdle from "hurdle";

let width = document.documentElement.scrollWidth;
let height = document.documentElement.scrollHeight;
console.log(`width: ${width}`);

hurdle.start("main", width, height)
