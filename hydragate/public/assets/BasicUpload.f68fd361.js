var B=Object.defineProperty,N=Object.defineProperties;var y=Object.getOwnPropertyDescriptors;var p=Object.getOwnPropertySymbols;var k=Object.prototype.hasOwnProperty,S=Object.prototype.propertyIsEnumerable;var d=(e,t,o)=>t in e?B(e,t,{enumerable:!0,configurable:!0,writable:!0,value:o}):e[t]=o,n=(e,t)=>{for(var o in t||(t={}))k.call(t,o)&&d(e,o,t[o]);if(p)for(var o of p(t))S.call(t,o)&&d(e,o,t[o]);return e},l=(e,t)=>N(e,y(t));import{d as defineComponent,o as openBlock,c as createBlock,a as createVNode,aY as __unplugin_components_10,aX as useGlobSetting,a0 as computed,u as useMessage,z as useDialog,b as reactive,ac as watch,a3 as toRefs,_ as _export_sfc,al as isString,aZ as ResultEnum,X as resolveComponent,h as createElementBlock,i as createBaseVNode,aD as Fragment,aA as renderList,aO as normalizeStyle,w as withCtx,au as mergeProps,at as createCommentVNode,l as createTextVNode,as as toDisplayString,N as NIcon,a_ as __unplugin_components_9,a7 as __unplugin_components_4,a$ as __unplugin_components_5}from"./index.3790d147.js";import{c as componentSetting}from"./componentSetting.f3f6bdde.js";import{P as PlusOutlined}from"./PlusOutlined.71de4daf.js";const _hoisted_1$2={xmlns:"http://www.w3.org/2000/svg","xmlns:xlink":"http://www.w3.org/1999/xlink",viewBox:"0 0 1024 1024"},_hoisted_2$2=createVNode("path",{d:"M360 184h-8c4.4 0 8-3.6 8-8v8h304v-8c0 4.4 3.6 8 8 8h-8v72h72v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80h72v-72zm504 72H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zM731.3 840H292.7l-24.2-512h487l-24.2 512z",fill:"currentColor"},null,-1);var DeleteOutlined=defineComponent({name:"DeleteOutlined",render:function(t,o){return openBlock(),createBlock("svg",_hoisted_1$2,[_hoisted_2$2])}});const _hoisted_1$1={xmlns:"http://www.w3.org/2000/svg","xmlns:xlink":"http://www.w3.org/1999/xlink",viewBox:"0 0 1024 1024"},_hoisted_2$1=createVNode("path",{d:"M942.2 486.2C847.4 286.5 704.1 186 512 186c-192.2 0-335.4 100.5-430.2 300.3a60.3 60.3 0 0 0 0 51.5C176.6 737.5 319.9 838 512 838c192.2 0 335.4-100.5 430.2-300.3c7.7-16.2 7.7-35 0-51.5zM512 766c-161.3 0-279.4-81.8-362.7-254C232.6 339.8 350.7 258 512 258c161.3 0 279.4 81.8 362.7 254C791.5 684.2 673.4 766 512 766zm-4-430c-97.2 0-176 78.8-176 176s78.8 176 176 176s176-78.8 176-176s-78.8-176-176-176zm0 288c-61.9 0-112-50.1-112-112s50.1-112 112-112s112 50.1 112 112s-50.1 112-112 112z",fill:"currentColor"},null,-1);var EyeOutlined=defineComponent({name:"EyeOutlined",render:function(t,o){return openBlock(),createBlock("svg",_hoisted_1$1,[_hoisted_2$1])}});const basicProps=l(n({},__unplugin_components_10.props),{accept:{type:String,default:".jpg,.png,.jpeg,.svg,.gif"},helpText:{type:String,default:""},maxSize:{type:Number,default:2},maxNumber:{type:Number,default:1/0},value:{type:Array,default:()=>[]},width:{type:Number,default:104},height:{type:Number,default:104}});var BasicUpload_vue_vue_type_style_index_0_lang="";const globSetting=useGlobSetting(),_sfc_main=defineComponent({name:"BasicUpload",components:{EyeOutlined,DeleteOutlined,PlusOutlined},props:n({},basicProps),emits:["uploadChange","delete"],setup(props,{emit}){const getCSSProperties=computed(()=>({width:`${props.width}px`,height:`${props.height}px`})),message=useMessage(),dialog=useDialog(),state=reactive({showModal:!1,previewUrl:"",originalImgList:[],imgList:[]});watch(()=>props.value,()=>{imgList.value=props.value.map(e=>getImgUrl(e))});function preview(e){state.showModal=!0,state.previewUrl=e}function remove(e){dialog.info({title:"\u63D0\u793A",content:"\u4F60\u786E\u5B9A\u8981\u5220\u9664\u5417\uFF1F",positiveText:"\u786E\u5B9A",negativeText:"\u53D6\u6D88",onPositiveClick:()=>{state.imgList.splice(e,1),state.originalImgList.splice(e,1),emit("uploadChange",state.originalImgList),emit("delete",state.originalImgList)},onNegativeClick:()=>{}})}function getImgUrl(e){const{imgUrl:t}=globSetting;return/(^http|https:\/\/)/g.test(e)?e:`${t}${e}`}function checkFileType(e){return componentSetting.upload.fileType.includes(e)}function beforeUpload({file:e}){const t=e.file,{maxSize:o,accept:i}=props,r=isString(i)&&i.split(",")||[];if(o&&t.size/1024/1024>=o)return message.error(`\u4E0A\u4F20\u6587\u4EF6\u6700\u5927\u503C\u4E0D\u80FD\u8D85\u8FC7${o}M`),!1;const c=componentSetting.upload.fileType;return r.length>0&&!checkFileType(t.type)?(message.error(`\u53EA\u80FD\u4E0A\u4F20\u6587\u4EF6\u7C7B\u578B\u4E3A${c.join(",")}`),!1):!0}function finish({event:Event}){const res=eval("("+Event.target.response+")"),infoField=componentSetting.upload.apiSetting.infoField,{code}=res,message=res.msg||res.message||"\u4E0A\u4F20\u5931\u8D25",result=res[infoField];if(code===ResultEnum.SUCCESS){let e=getImgUrl(result.photo);state.imgList.push(e),state.originalImgList.push(result.photo),emit("uploadChange",state.originalImgList)}else message.error(message)}return l(n({},toRefs(state)),{finish,preview,remove,beforeUpload,getCSSProperties})}}),_hoisted_1={class:"w-full"},_hoisted_2={class:"upload"},_hoisted_3={class:"upload-card"},_hoisted_4={class:"upload-card-item-info"},_hoisted_5={class:"img-box"},_hoisted_6=["src"],_hoisted_7={class:"img-box-actions"},_hoisted_8={class:"flex flex-col justify-center"},_hoisted_9=createBaseVNode("span",{class:"upload-title"},"\u4E0A\u4F20\u56FE\u7247",-1),_hoisted_10=["src"];function _sfc_render(e,t,o,i,r,c){const _=resolveComponent("EyeOutlined"),a=NIcon,m=resolveComponent("DeleteOutlined"),g=resolveComponent("PlusOutlined"),h=__unplugin_components_10,f=__unplugin_components_9,v=__unplugin_components_4,w=__unplugin_components_5;return openBlock(),createElementBlock(Fragment,null,[createBaseVNode("div",_hoisted_1,[createBaseVNode("div",_hoisted_2,[createBaseVNode("div",_hoisted_3,[(openBlock(!0),createElementBlock(Fragment,null,renderList(e.imgList,(s,u)=>(openBlock(),createElementBlock("div",{class:"upload-card-item",style:normalizeStyle(e.getCSSProperties),key:`img_${u}`},[createBaseVNode("div",_hoisted_4,[createBaseVNode("div",_hoisted_5,[createBaseVNode("img",{src:s},null,8,_hoisted_6)]),createBaseVNode("div",_hoisted_7,[createVNode(a,{size:"18",class:"mx-2 action-icon",onClick:C=>e.preview(s)},{default:withCtx(()=>[createVNode(_)]),_:2},1032,["onClick"]),createVNode(a,{size:"18",class:"mx-2 action-icon",onClick:C=>e.remove(u)},{default:withCtx(()=>[createVNode(m)]),_:2},1032,["onClick"])])])],4))),128)),e.imgList.length<e.maxNumber?(openBlock(),createElementBlock("div",{key:0,class:"upload-card-item upload-card-item-select-picture",style:normalizeStyle(e.getCSSProperties)},[createVNode(h,mergeProps(e.$props,{"file-list-style":{display:"none"},onBeforeUpload:e.beforeUpload,onFinish:e.finish}),{default:withCtx(()=>[createBaseVNode("div",_hoisted_8,[createVNode(a,{size:"18",class:"m-auto"},{default:withCtx(()=>[createVNode(g)]),_:1}),_hoisted_9])]),_:1},16,["onBeforeUpload","onFinish"])],4)):createCommentVNode("",!0)])]),createVNode(v,null,{default:withCtx(()=>[e.helpText?(openBlock(),createBlock(f,{key:0,title:"\u63D0\u793A",type:"info",class:"flex w-full"},{default:withCtx(()=>[createTextVNode(toDisplayString(e.helpText),1)]),_:1})):createCommentVNode("",!0)]),_:1})]),createVNode(w,{show:e.showModal,"onUpdate:show":t[0]||(t[0]=s=>e.showModal=s),preset:"card",title:"\u9884\u89C8",bordered:!1,style:{width:"520px"}},{default:withCtx(()=>[createBaseVNode("img",{src:e.previewUrl},null,8,_hoisted_10)]),_:1},8,["show"])],64)}var BasicUpload=_export_sfc(_sfc_main,[["render",_sfc_render]]);export{BasicUpload as B};
