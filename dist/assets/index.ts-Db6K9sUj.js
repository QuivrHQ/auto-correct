import{g as W,o as D}from"./storage-Lryb-ELa.js";import{a as U}from"./language-tool-client-CaKIfVxh.js";class _{element;overlay=null;shadowRoot=null;resizeObserver=null;tooltip=null;currentMatches=[];ignoredMatches=new Set;callbacks=null;boundHideTooltip;constructor(t){this.element=t,this.boundHideTooltip=this.handleOutsideClick.bind(this)}init(t){this.callbacks=t,this.createOverlay(),this.setupObservers()}createOverlay(){this.overlay=document.createElement("div"),this.overlay.className="autocorrect-overlay",this.overlay.style.cssText=`
      position: absolute;
      pointer-events: none;
      overflow: visible;
      z-index: 2147483646;
    `,this.shadowRoot=this.overlay.attachShadow({mode:"open"});const t=document.createElement("style");t.textContent=`
      :host {
        all: initial;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      }

      /* Error underlines - now clickable */
      .error-highlight {
        position: absolute;
        background: transparent;
        cursor: pointer;
        pointer-events: auto;
        border-radius: 2px;
      }
      .error-highlight:hover {
        background: rgba(239, 68, 68, 0.1);
      }
      .error-highlight::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: currentColor;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='6' height='3' viewBox='0 0 6 3'%3E%3Cpath d='M0 3 L1.5 0 L3 3 L4.5 0 L6 3' stroke='%23EF4444' fill='none' stroke-width='1'/%3E%3C/svg%3E");
        background-repeat: repeat-x;
        background-position: bottom;
        background-size: 6px 3px;
      }
      .error-spelling::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='6' height='3' viewBox='0 0 6 3'%3E%3Cpath d='M0 3 L1.5 0 L3 3 L4.5 0 L6 3' stroke='%23EF4444' fill='none' stroke-width='1'/%3E%3C/svg%3E");
      }
      .error-grammar::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='6' height='3' viewBox='0 0 6 3'%3E%3Cpath d='M0 3 L1.5 0 L3 3 L4.5 0 L6 3' stroke='%23F59E0B' fill='none' stroke-width='1'/%3E%3C/svg%3E");
      }
      .error-grammar:hover {
        background: rgba(245, 158, 11, 0.1);
      }

      /* Tooltip */
      .tooltip {
        position: fixed;
        background: white;
        border-radius: 12px;
        box-shadow: 0 4px 24px rgba(0, 0, 0, 0.12), 0 0 0 1px rgba(0, 0, 0, 0.05);
        padding: 0;
        min-width: 240px;
        max-width: 320px;
        z-index: 2147483647;
        animation: tooltipIn 0.15s ease-out;
        overflow: hidden;
        pointer-events: auto;
      }
      @keyframes tooltipIn {
        from {
          opacity: 0;
          transform: translateY(-4px);
        }
        to {
          opacity: 1;
          transform: translateY(0);
        }
      }

      .tooltip-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 10px 12px;
        background: #FAFAFA;
        border-bottom: 1px solid #F0F0F0;
      }
      .tooltip-category {
        display: flex;
        align-items: center;
        gap: 8px;
        font-weight: 600;
        font-size: 13px;
        color: #374151;
      }
      .category-dot {
        width: 8px;
        height: 8px;
        border-radius: 50%;
      }
      .category-dot.spelling { background: #EF4444; }
      .category-dot.grammar { background: #F59E0B; }
      .category-dot.style { background: #3B82F6; }

      .tooltip-close {
        width: 24px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: transparent;
        color: #9CA3AF;
        cursor: pointer;
        border-radius: 4px;
        transition: all 0.15s;
      }
      .tooltip-close:hover {
        background: #E5E7EB;
        color: #374151;
      }

      .tooltip-body {
        padding: 12px;
      }
      .tooltip-message {
        color: #4B5563;
        font-size: 13px;
        line-height: 1.5;
        margin-bottom: 12px;
      }

      .tooltip-suggestions {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
      }
      .suggestion-btn {
        padding: 7px 14px;
        background: #3B82F6;
        color: white;
        border: none;
        border-radius: 8px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s;
        pointer-events: auto;
      }
      .suggestion-btn:hover {
        background: #2563EB;
        transform: translateY(-1px);
      }
      .suggestion-btn:active {
        transform: translateY(0);
      }

      .ignore-btn {
        padding: 7px 14px;
        background: white;
        color: #6B7280;
        border: 1px solid #E5E7EB;
        border-radius: 8px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s;
        pointer-events: auto;
      }
      .ignore-btn:hover {
        background: #F9FAFB;
        border-color: #D1D5DB;
      }
    `,this.shadowRoot.appendChild(t),document.body.appendChild(this.overlay),this.updatePosition()}setupObservers(){this.resizeObserver=new ResizeObserver(()=>{this.updatePosition()}),this.resizeObserver.observe(this.element),this.element.addEventListener("scroll",()=>{this.updatePosition(),this.hideTooltip()}),window.addEventListener("scroll",()=>{this.updatePosition(),this.hideTooltip()},!0),window.addEventListener("resize",()=>{this.updatePosition(),this.hideTooltip()}),this.element.addEventListener("input",()=>{this.hideTooltip()})}updatePosition(){if(!this.overlay)return;const t=this.element.getBoundingClientRect(),e=window.scrollX,n=window.scrollY;this.overlay.style.left=`${t.left+e}px`,this.overlay.style.top=`${t.top+n}px`,this.overlay.style.width=`${t.width}px`,this.overlay.style.height=`${t.height}px`}render(t,e){if(!this.shadowRoot||(this.currentMatches=t,this.hideTooltip(),this.shadowRoot.querySelectorAll(".error-highlight").forEach(l=>l.remove()),console.log("[AutoCorrect] Rendering",t.length,"matches"),t.length===0))return;const r=this.getVisibleTextRange(e),s=t.filter(l=>{const c=l.offset+l.length,d=Math.max(0,r.startOffset-500),h=r.endOffset+500;return c>d&&l.offset<h});console.log("[AutoCorrect] Visible matches:",s.length,"range:",r);const a=this.calculatePositions(s,e);console.log("[AutoCorrect] Positions calculated:",a);let i=0;a.forEach((l,c)=>{const d=s[c],h=t.indexOf(d),g=`${d.offset}-${d.length}-${d.rule.id}`;if(this.ignoredMatches.has(g)){console.log("[AutoCorrect] Skipping ignored match:",g);return}if(l.y<-50||l.y>this.element.clientHeight+50){console.log("[AutoCorrect] Skipping out-of-bounds match:",l.y,"element height:",this.element.clientHeight);return}const u=document.createElement("span");u.className=`error-highlight ${this.getErrorClass(d)}`,u.style.left=`${l.x}px`,u.style.top=`${l.y}px`,u.style.width=`${l.width}px`,u.style.height=`${l.height}px`,u.dataset.matchIndex=String(h),u.addEventListener("click",b=>{b.preventDefault(),b.stopPropagation(),this.showTooltip(d,b.clientX,b.clientY,l)}),this.shadowRoot.appendChild(u),i++}),console.log("[AutoCorrect] Rendered",i,"underlines")}getVisibleTextRange(t){const e=this.element;if(e instanceof HTMLInputElement)return{startOffset:0,endOffset:t.length};if(e instanceof HTMLTextAreaElement){const n=window.getComputedStyle(e),r=parseFloat(n.lineHeight)||parseFloat(n.fontSize)*1.2,s=e.scrollTop,a=e.clientHeight,i=Math.max(0,Math.floor(s/r)-2),l=Math.ceil((s+a)/r)+2,c=t.split(`
`);let d=0,h=t.length;for(let u=0;u<i&&u<c.length;u++)d+=c[u].length+1;let g=0;for(let u=0;u<=l&&u<c.length;u++)g+=c[u].length+1;return h=Math.min(g,t.length),{startOffset:d,endOffset:h}}return{startOffset:0,endOffset:t.length}}getErrorClass(t){const e=t.rule.category.id.toUpperCase();return e.includes("TYPO")||e.includes("SPELL")?"error-spelling":(e.includes("GRAMMAR"),"error-grammar")}getCategoryInfo(t){const e=t.rule.category.id.toUpperCase();return e.includes("TYPO")||e.includes("SPELL")?{name:"Orthographe",class:"spelling"}:e.includes("GRAMMAR")?{name:"Grammaire",class:"grammar"}:{name:"Style",class:"style"}}showTooltip(t,e,n,r){if(this.hideTooltip(),!this.shadowRoot)return;const s=this.getCategoryInfo(t);this.tooltip=document.createElement("div"),this.tooltip.className="tooltip";const a=window.innerHeight,i=150,l=a-n-20;let c=n+10;l<i&&n>i&&(c=n-i-10),this.tooltip.style.left=`${Math.min(e-20,window.innerWidth-340)}px`,this.tooltip.style.top=`${c}px`,this.tooltip.innerHTML=`
      <div class="tooltip-header">
        <div class="tooltip-category">
          <span class="category-dot ${s.class}"></span>
          <span>${s.name}</span>
        </div>
        <button class="tooltip-close" aria-label="Fermer">
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M1 1L13 13M1 13L13 1"/>
          </svg>
        </button>
      </div>
      <div class="tooltip-body">
        <p class="tooltip-message">${t.message}</p>
        <div class="tooltip-suggestions">
          ${t.replacements.slice(0,3).map(d=>`<button class="suggestion-btn" data-replacement="${this.escapeHtml(d.value)}">${this.escapeHtml(d.value)}</button>`).join("")}
          <button class="ignore-btn">Ignorer</button>
        </div>
      </div>
    `,this.tooltip.querySelector(".tooltip-close")?.addEventListener("click",()=>{this.hideTooltip()}),this.tooltip.querySelectorAll(".suggestion-btn").forEach(d=>{d.addEventListener("click",h=>{h.stopPropagation();const g=h.target.dataset.replacement||"";console.log("[AutoCorrect] Suggestion clicked:",g),this.callbacks?.onReplace(t,g),this.hideTooltip()})}),this.tooltip.querySelector(".ignore-btn")?.addEventListener("click",()=>{const d=`${t.offset}-${t.length}-${t.rule.id}`;this.ignoredMatches.add(d),this.callbacks?.onIgnore(t),this.hideTooltip(),this.render(this.currentMatches,this.getElementText())}),this.shadowRoot.appendChild(this.tooltip),setTimeout(()=>{document.addEventListener("click",this.boundHideTooltip,!1)},10)}handleOutsideClick(t){const e=t.composedPath();this.tooltip&&!e.includes(this.tooltip)&&this.hideTooltip()}hideTooltip(){this.tooltip&&(this.tooltip.remove(),this.tooltip=null,document.removeEventListener("click",this.boundHideTooltip,!1))}escapeHtml(t){const e=document.createElement("div");return e.textContent=t,e.innerHTML}getElementText(){return this.element instanceof HTMLInputElement||this.element instanceof HTMLTextAreaElement?this.element.value:this.element.textContent||""}calculatePositions(t,e){const n=this.element instanceof HTMLInputElement,r=this.element instanceof HTMLTextAreaElement;return n||r?this.calculateInputPositions(t,e):this.calculateContentEditablePositions(t,e)}calculateInputPositions(t,e){const n=this.element,r=window.getComputedStyle(n),s=document.createElement("div");s.style.cssText=`
      position: absolute;
      top: -9999px;
      left: -9999px;
      visibility: hidden;
      white-space: pre-wrap;
      word-wrap: break-word;
      overflow-wrap: break-word;
      font-family: ${r.fontFamily};
      font-size: ${r.fontSize};
      font-weight: ${r.fontWeight};
      font-style: ${r.fontStyle};
      letter-spacing: ${r.letterSpacing};
      word-spacing: ${r.wordSpacing};
      line-height: ${r.lineHeight};
      text-transform: ${r.textTransform};
      padding: ${r.padding};
      border: ${r.borderWidth} solid transparent;
      box-sizing: border-box;
      width: ${n.offsetWidth}px;
    `,document.body.appendChild(s);const a=[];parseFloat(r.paddingLeft);const i=parseFloat(r.paddingTop)||0;parseFloat(r.borderLeftWidth);const l=parseFloat(r.borderTopWidth)||0,c=parseFloat(r.lineHeight)||parseFloat(r.fontSize)*1.2,d=n.scrollLeft||0,h=n.scrollTop||0;return t.forEach(g=>{const u=e.substring(0,g.offset),b=e.substring(g.offset,g.offset+g.length);s.innerHTML="";const p=u.split(`
`),v=p.length-1,f=p[v],m=document.createElement("span");m.textContent=f,s.appendChild(m);const T=document.createElement("span");T.textContent=b,s.appendChild(T),m.getBoundingClientRect();const y=T.getBoundingClientRect(),$=s.getBoundingClientRect(),N=y.left-$.left-d,B=y.width||T.offsetWidth||10,z=i+l+v*c-h;a.push({x:Math.max(0,N),y:z,width:Math.max(B,10),height:c})}),document.body.removeChild(s),a}calculateContentEditablePositions(t,e){const n=[],r=this.element,s=r.getBoundingClientRect(),a=r.textContent||"";return t.forEach(i=>{try{const l=e.substring(i.offset,i.offset+i.length);let c=Math.max(0,i.offset-50),d=Math.min(a.length,i.offset+i.length+50),h=a.substring(c,d),g=h.indexOf(l);if(g===-1&&(g=h.toLowerCase().indexOf(l.toLowerCase())),g!==-1){const u=c+g,b=document.createRange(),p=this.findTextNodeByOffset(r,u,i.length);if(p){b.setStart(p.node,p.offset);const v=Math.min(p.offset+i.length,p.node.textContent?.length||0);b.setEnd(p.node,v);const f=b.getClientRects();if(f.length>0){const m=f[0];n.push({x:m.left-s.left+r.scrollLeft,y:m.top-s.top+r.scrollTop,width:Math.max(m.width,10),height:m.height})}}}}catch(l){console.log("[AutoCorrect] Position calculation error:",l)}}),n}findTextNodeByOffset(t,e,n){const r=document.createTreeWalker(t,NodeFilter.SHOW_TEXT);let s=0;for(;r.nextNode();){const a=r.currentNode,l=(a.textContent||"").length;if(s+l>e){const c=e-s;return c+n<=l?{node:a,offset:c}:{node:a,offset:c}}s+=l}return null}destroy(){this.hideTooltip(),this.resizeObserver&&this.resizeObserver.disconnect(),this.overlay&&this.overlay.remove()}}const O=400,A=new WeakMap;let C=null,x=null;function R(o){C=o,o.enabled||document.querySelectorAll(".autocorrect-overlay").forEach(t=>t.remove())}function L(o){if(o instanceof HTMLInputElement){const t=o.type.toLowerCase();return!(!["text","search","email","url","tel",""].includes(t)||o.offsetWidth<100)}if(o instanceof HTMLTextAreaElement)return!0;if(o instanceof HTMLElement&&o.isContentEditable){if(o.offsetWidth<100||o.offsetHeight<30)return!1;const t=o.getAttribute("role");return!(t&&["button","menuitem","option","tab"].includes(t))}return!1}function E(o){return o instanceof HTMLInputElement||o instanceof HTMLTextAreaElement?o.value:o.textContent||""}function q(o,t=500){const e=E(o);if(e.length<=t)return{text:e,offset:0};let n=0;if(o instanceof HTMLInputElement||o instanceof HTMLTextAreaElement)n=o.selectionStart||0;else{const c=window.getSelection();if(c&&c.rangeCount>0){const d=c.getRangeAt(0),h=document.createRange();h.selectNodeContents(o),h.setEnd(d.startContainer,d.startOffset),n=h.toString().length}}const r=e.lastIndexOf(`

`,n),s=e.indexOf(`

`,n);let a=r===-1?0:r+2,i=s===-1?e.length:s;const l=Math.floor(t/2);for(i-a<t&&(a=Math.max(0,n-l),i=Math.min(e.length,n+l));a>0&&e[a-1]!==" "&&e[a-1]!==`
`;)a--;for(;i<e.length&&e[i]!==" "&&e[i]!==`
`;)i++;return{text:e.substring(a,i),offset:a}}function P(o,t,e,n){if(console.log("[AutoCorrect] setTextContent called:",{offset:t,length:e,replacement:n,elementType:o.tagName}),o instanceof HTMLInputElement||o instanceof HTMLTextAreaElement){const r=o.value;console.log("[AutoCorrect] Input/Textarea replacement:",{textLength:r.length,beforeOffset:t});const s=r.substring(0,t),a=r.substring(t+e);o.value=s+n+a;const i=t+n.length;o.setSelectionRange(i,i),o.dispatchEvent(new Event("input",{bubbles:!0})),console.log("[AutoCorrect] Replacement done for input/textarea")}else{console.log("[AutoCorrect] Contenteditable replacement at offset:",t);const s=(o.textContent||"").substring(t,t+e);console.log("[AutoCorrect] Looking for text:",s,"at offset:",t);const a=document.createTreeWalker(o,NodeFilter.SHOW_TEXT);let i=0,l=!1;for(;a.nextNode();){const c=a.currentNode,d=c.textContent||"",h=d.length;if(i+h>t){const g=t-i;console.log("[AutoCorrect] Found node:",{nodeText:d.substring(0,50),nodeOffset:g,nodeLength:h});const u=o.classList.contains("ck-editor__editable")||o.classList.contains("ck-content"),b=c,p=g,v=Math.min(e,h-g);if(u){console.log("[AutoCorrect] CKEditor detected, using paste simulation");const f=window.getSelection();if(f){const m=document.createRange();m.setStart(b,p),m.setEnd(b,p+v),f.removeAllRanges(),f.addRange(m),console.log("[AutoCorrect] Selection set on CKEditor:",{startOffset:p,endOffset:p+v,selectedText:f.toString()}),setTimeout(async()=>{try{f.removeAllRanges(),f.addRange(m),await navigator.clipboard.writeText(n),console.log("[AutoCorrect] Clipboard written, triggering paste");const T=new ClipboardEvent("paste",{bubbles:!0,cancelable:!0,clipboardData:new DataTransfer});T.clipboardData?.setData("text/plain",n);const y=o.dispatchEvent(T);console.log("[AutoCorrect] Paste event dispatched, handled:",y),(!y||T.defaultPrevented)&&document.execCommand("insertText",!1,n)}catch(T){console.error("[AutoCorrect] Paste simulation failed:",T),document.execCommand("insertText",!1,n)}},10)}}else o.focus(),setTimeout(()=>{try{const f=window.getSelection();if(f){const m=document.createRange();m.setStart(b,p),m.setEnd(b,p+v),f.removeAllRanges(),f.addRange(m),console.log("[AutoCorrect] Selection set:",{startOffset:p,endOffset:p+v,selectedText:f.toString()}),document.execCommand("insertText",!1,n)?console.log("[AutoCorrect] Replacement done via execCommand"):(console.log("[AutoCorrect] execCommand failed, trying delete + insertText"),document.execCommand("delete",!1),document.execCommand("insertText",!1,n))}}catch(f){console.error("[AutoCorrect] Error during replacement:",f)}},10);l=!0;break}i+=h}l||console.warn("[AutoCorrect] Could not find text node at offset:",t),o.dispatchEvent(new InputEvent("input",{bubbles:!0,inputType:"insertText",data:n}))}}function H(o,t,e){e.debounceTimer!==null&&clearTimeout(e.debounceTimer),e.debounceTimer=window.setTimeout(()=>{e.debounceTimer=null,o()},t)}async function M(o){if(!C?.enabled||!C?.apiUrl){console.log("[AutoCorrect] Disabled or no API URL");return}const t=E(o.element);if(t===o.lastText){console.log("[AutoCorrect] Text unchanged, skipping");return}if(o.lastText=t,t.trim().length<3){o.currentMatches=[],o.renderer.render([],t);return}const{text:e,offset:n}=q(o.element,500);console.log("[AutoCorrect] Checking text:",e.substring(0,50),"... (",e.length,"of",t.length,"chars, offset:",n,")"),console.log("[AutoCorrect] Calling API...");const s=(await U(e,C.language,C.apiUrl)).map(a=>({...a,offset:a.offset+n}));console.log("[AutoCorrect] Got",s.length,"matches"),o.currentMatches=s,o.renderer.render(s,t)}function w(o){if(A.has(o))return;console.log("[AutoCorrect] Attaching to field:",o.tagName,o.className?.substring?.(0,50));const t=new _(o),e={element:o,renderer:t,debounceTimer:null,lastText:"",currentMatches:[]};t.init({onReplace:(n,r)=>{P(o,n.offset,n.length,r)},onIgnore:n=>{}}),A.set(o,e),o.addEventListener("input",()=>{H(()=>M(e),O,e)}),o.addEventListener("focus",()=>{x=e,E(o).trim().length>=3&&H(()=>M(e),O,e)}),o.addEventListener("blur",()=>{}),document.activeElement===o&&(x=e,E(o).trim().length>=3&&setTimeout(()=>M(e),500))}function k(o){const t=A.get(o);t&&(t.debounceTimer!==null&&clearTimeout(t.debounceTimer),t.renderer.destroy(),A.delete(o))}function S(){const o=document.querySelectorAll('input[type="text"], input[type="search"], input[type="email"], input[type="url"], input[type="tel"], input:not([type])'),t=document.querySelectorAll("textarea"),e=document.querySelectorAll('[contenteditable]:not([contenteditable="false"])');console.log("[AutoCorrect] Scan found:",o.length,"inputs,",t.length,"textareas,",e.length,"contenteditables"),o.forEach(n=>{L(n)&&w(n)}),t.forEach(n=>{w(n)}),e.forEach(n=>{n instanceof HTMLElement&&w(n)})}function F(){S(),new MutationObserver(t=>{t.forEach(e=>{if(e.type==="attributes"&&e.attributeName==="contenteditable"){const n=e.target;n instanceof HTMLElement&&L(n)&&(console.log("[AutoCorrect] Contenteditable attribute changed on:",n.tagName,n.className?.substring?.(0,50)),w(n))}e.addedNodes.forEach(n=>{n instanceof Element&&(L(n)&&w(n),n.querySelectorAll('input, textarea, [contenteditable]:not([contenteditable="false"])').forEach(r=>{L(r)&&w(r)}))}),e.removedNodes.forEach(n=>{n instanceof Element&&(k(n),n.querySelectorAll("input, textarea, [contenteditable]").forEach(k))})})}).observe(document.body,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["contenteditable"]}),setTimeout(S,2e3),setTimeout(S,5e3),G()}function G(){chrome.runtime.onMessage.addListener((o,t,e)=>{if(o.type==="GET_MATCHES"){const n={type:"MATCHES_RESPONSE",matches:x?.currentMatches||[],textLength:x?E(x.element).length:0,fieldInfo:x?{tagName:x.element.tagName.toLowerCase(),hasContent:E(x.element).trim().length>0}:null};return e(n),!0}if(o.type==="APPLY_SUGGESTION"){const n=o;if(x&&x.currentMatches[n.matchIndex]){const r=x.currentMatches[n.matchIndex];P(x.element,r.offset,r.length,n.replacement),e({type:"SUGGESTION_APPLIED",success:!0})}else e({type:"SUGGESTION_APPLIED",success:!1});return!0}return!1})}function Y(){document.querySelectorAll('input, textarea, [contenteditable="true"]').forEach(k)}async function I(){const o=await W();R(o),o.enabled&&F(),D(t=>{R(t),t.enabled?F():Y()})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",I):I();
