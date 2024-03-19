const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let imgDivEl;
let nameEl;
let infoEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  var obj=await invoke("files");
  console.log(obj);
  var maps=obj.maps;
  var name=obj.name;
  nameEl.textContent=name;
  //清空imgDivEl
  imgDivEl.innerHTML="";
  //循环map数组，创建img标签
  let index = 0;
  for (const key in maps) {
    index++;
    //创建img标签
    var img=document.createElement("img");
    //设置img的src属性,base64格式的图片
    img.src="data:image/png;base64,"+`${maps[key]}`;
    img.style.width="200px";
    img.style.height="150px";
    img.className="logo tauri";
    img.id="img_"+index;
    //创建a标签
    var a=document.createElement("a");
    a.href="javascript:void(0)";
    a.id="a_"+index;
    a.setAttribute("data",key);
    //给a标签添加点击事件，如果点击后a的value值等于 name值，就提示正确，否则提示错误
    a.onclick = function() {
      infoEl.textContent = "";
      if(this.getAttribute("data") === name){
        infoEl.textContent = "回答正确";
      }else{
        infoEl.textContent = "回答错误";
      }
    };
    //把img标签添加到a标签中
    a.appendChild(img);
    imgDivEl.appendChild(a);
  }
  
}

window.addEventListener("DOMContentLoaded", () => {
  imgDivEl = document.querySelector("#img_div");
  nameEl = document.querySelector("#name");
  infoEl = document.querySelector("#info");
  greet();
});
