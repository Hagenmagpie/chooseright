const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let imgDivEl;
let nameEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  var obj=await invoke("files");
  // console.log(obj);
  var list=obj.list;
  var name=obj.name;
  nameEl.textContent=name;
  //清空imgDivEl
  imgDivEl.innerHTML="";
  //循环list数组，创建img标签
  for (var i=0;i<list.length;i++){
    var img=document.createElement("img");
    //设置img的src属性,base64格式的图片
    img.src="data:image/png;base64,"+list[i];
    img.style.width="200px";
    img.style.height="150px";
    //class="logo tauri"
    img.className="logo tauri";
    imgDivEl.appendChild(img);
  }
  
}

window.addEventListener("DOMContentLoaded", () => {
  imgDivEl = document.querySelector("#img_div");
  nameEl = document.querySelector("#name");
  greet();
});
