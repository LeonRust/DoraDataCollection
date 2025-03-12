var Q=Object.defineProperty;var b=Object.getOwnPropertySymbols;var J=Object.prototype.hasOwnProperty,W=Object.prototype.propertyIsEnumerable;var E=(n,t,u)=>t in n?Q(n,t,{enumerable:!0,configurable:!0,writable:!0,value:u}):n[t]=u,g=(n,t)=>{for(var u in t||(t={}))J.call(t,u)&&E(n,u,t[u]);if(b)for(var u of b(t))W.call(t,u)&&E(n,u,t[u]);return n};var B=(n,t,u)=>new Promise((m,h)=>{var d=l=>{try{i(u.next(l))}catch(c){h(c)}},v=l=>{try{i(u.throw(l))}catch(c){h(c)}},i=l=>l.done?m(l.value):Promise.resolve(l.value).then(d,v);i((u=u.apply(n,t)).next())});import{T as C,B as X}from"./TableAction.4151f0b4.js";import{aF as Y,v as T,y as Z,d as ee,f as ue,r as s,u as te,b as w,aw as ne,dO as ae,o as oe,h as le,i as x,a,w as o,j as f,l as y,as as A,dR as se,A as ie,N as ce,q as re,dQ as de,a7 as pe,a$ as _e}from"./index.3790d147.js";import{P as fe}from"./PlusOutlined.71de4daf.js";import"./sortable.esm.1502e2e1.js";import"./useDesignSetting.e02d28c9.js";import"./propTypes.15d30787.js";import"./FormOutlined.88898c98.js";import"./componentSetting.f3f6bdde.js";import"./index.esm.ed31f38f.js";import"./DownOutlined.25d3f8ea.js";function me(){return Y.request({url:"/role/list",method:"GET"})}const he=[{title:"id",key:"id"},{title:"\u89D2\u8272\u540D\u79F0",key:"name"},{title:"\u8BF4\u660E",key:"explain"},{title:"\u662F\u5426\u9ED8\u8BA4\u89D2\u8272",key:"isDefault",render(n){return T(Z,{type:n.isDefault?"success":"error"},{default:()=>n.isDefault?"\u662F":"\u5426"})}},{title:"\u521B\u5EFA\u65F6\u95F4",key:"create_date"}],ve={class:"n-layout-page-header"},ye=y(" \u9875\u9762\u6570\u636E\u4E3A Mock \u793A\u4F8B\u6570\u636E\uFF0C\u975E\u771F\u5B9E\u6570\u636E\u3002 "),ke=y(" \u6DFB\u52A0\u89D2\u8272 "),Fe={class:"py-3 menu-list"},ge=y("\u63D0\u4EA4"),Me=ee({setup(n){const t=ue(),u=s(null),m=te(),h=s(),d=s(!1),v=s(!1),i=s(!1),l=s(""),c=s([]),r=s([]),p=s(["console","step-form"]),R=w({pageSize:5,name:"xiaoMa"}),K=w({width:250,title:"\u64CD\u4F5C",key:"action",fixed:"right",render(e){return T(C,{style:"button",actions:[{label:"\u83DC\u5355\u6743\u9650",onClick:V.bind(null,e),ifShow:()=>!0,auth:["basic_list"]},{label:"\u7F16\u8F91",onClick:U.bind(null,e),ifShow:()=>!0,auth:["basic_list"]},{label:"\u5220\u9664",icon:"ic:outline-delete-outline",onClick:q.bind(null,e),ifShow:()=>!0,auth:["basic_list"]}]})}}),M=e=>B(this,null,function*(){return g(g({},f(R)),e),yield me()});function N(e){console.log(e)}function S(){h.value.reload()}function L(e){e.preventDefault(),v.value=!0,u.value.validate(_=>{_?m.error("\u8BF7\u586B\u5199\u5B8C\u6574\u4FE1\u606F"):(m.success("\u65B0\u5EFA\u6210\u529F"),setTimeout(()=>{d.value=!1,S()})),v.value=!1})}function U(e){console.log("\u70B9\u51FB\u4E86\u7F16\u8F91",e),t.push({name:"basic-info",params:{id:e.id}})}function q(e){console.log("\u70B9\u51FB\u4E86\u5220\u9664",e),m.info("\u70B9\u51FB\u4E86\u5220\u9664")}function V(e){l.value=`\u5206\u914D ${e.name} \u7684\u83DC\u5355\u6743\u9650`,p.value=e.menu_keys,d.value=!0}function H(e){p.value=[p.value,...e]}function O(e){r.value=e}function P(){r.value.length?r.value=[]:r.value=c.value.map(e=>e.key)}function $(){i.value?(p.value=[],i.value=!1):(p.value=se(c.value),i.value=!0)}return ne(()=>B(this,null,function*(){const e=yield ae();r.value=e.list.map(_=>_.key),c.value=e.list})),(e,_)=>{const D=ie,j=ce,k=re,z=de,G=pe,I=_e;return oe(),le("div",null,[x("div",ve,[a(D,{bordered:!1,title:"\u89D2\u8272\u6743\u9650\u7BA1\u7406"},{default:o(()=>[ye]),_:1})]),a(D,{bordered:!1,class:"mt-4 proCard"},{default:o(()=>[a(f(X),{columns:f(he),request:M,"row-key":F=>F.id,ref_key:"actionRef",ref:h,actionColumn:f(K),"onUpdate:checkedRowKeys":N},{tableTitle:o(()=>[a(k,{type:"primary"},{icon:o(()=>[a(j,null,{default:o(()=>[a(f(fe))]),_:1})]),default:o(()=>[ke]),_:1})]),action:o(()=>[a(f(C))]),_:1},8,["columns","row-key","actionColumn"])]),_:1}),a(I,{show:d.value,"onUpdate:show":_[0]||(_[0]=F=>d.value=F),"show-icon":!1,preset:"dialog",title:l.value},{action:o(()=>[a(G,null,{default:o(()=>[a(k,{type:"info",ghost:"","icon-placement":"left",onClick:P},{default:o(()=>[y(" \u5168\u90E8"+A(r.value.length?"\u6536\u8D77":"\u5C55\u5F00"),1)]),_:1}),a(k,{type:"info",ghost:"","icon-placement":"left",onClick:$},{default:o(()=>[y(" \u5168\u90E8"+A(i.value?"\u53D6\u6D88":"\u9009\u62E9"),1)]),_:1}),a(k,{type:"primary",loading:v.value,onClick:L},{default:o(()=>[ge]),_:1},8,["loading"])]),_:1})]),default:o(()=>[x("div",Fe,[a(z,{"block-line":"",cascade:"",checkable:"","virtual-scroll":!0,data:c.value,expandedKeys:r.value,"checked-keys":p.value,style:{"max-height":"950px",overflow:"hidden"},"onUpdate:checkedKeys":H,"onUpdate:expandedKeys":O},null,8,["data","expandedKeys","checked-keys"])])]),_:1},8,["show","title"])])}}});export{Me as default};
