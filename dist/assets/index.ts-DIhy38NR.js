import{a as W,g as Y,o as _}from"./storage-Dq_-2h6f.js";import{a as Q}from"./language-tool-client-B4BnbzYn.js";function j(n){const t=new Map,e=n.innerText||"";if(e.length===0)return t;const o=document.createTreeWalker(n,NodeFilter.SHOW_TEXT,{acceptNode:a=>{const l=a.parentElement;if(l){const h=l.tagName.toUpperCase();if(h==="SCRIPT"||h==="STYLE"||h==="NOSCRIPT"||h==="TEMPLATE")return NodeFilter.FILTER_REJECT;try{const r=window.getComputedStyle(l);if(r.display==="none"||r.visibility==="hidden")return NodeFilter.FILTER_REJECT}catch{}}return NodeFilter.FILTER_ACCEPT}});let s=0,i;for(;i=o.nextNode();){const a=i.textContent||"";for(let l=0;l<a.length;l++){for(;s<e.length&&e[s]===`
`&&a[l]!==`
`;)s++;s<e.length&&(t.set(s,{node:i,nodeOffset:l}),s++)}}return t}function C(n,t){const e=n.innerText||"";if(t<0||t>e.length)return null;const o=j(n),s=o.get(t);if(s)return{node:s.node,offset:s.nodeOffset};let i=-1;for(const a of o.keys())a<=t&&a>i&&(i=a);if(i>=0){const a=o.get(i),l=t-i,h=a.node.textContent?.length||0;return{node:a.node,offset:Math.min(a.nodeOffset+l,h)}}return null}function K(n,t,e){const o=C(n,t);if(!o)return console.warn("[AutoCorrect] Could not find start position for offset:",t),null;const s=C(n,t+e);if(!s){const a=C(n,t+e-1);if(!a)return console.warn("[AutoCorrect] Could not find end position for offset:",t+e),null;const l=document.createRange();return l.setStart(o.node,o.offset),l.setEnd(a.node,Math.min(a.offset+1,a.node.length)),l}const i=document.createRange();return i.setStart(o.node,o.offset),i.setEnd(s.node,s.offset),i}function R(n){const t=new Map,e=n.innerText||"";for(let o=0;o<e.length;o++){const s=C(n,o);s&&t.set(o,s)}return t}function w(n,t){return n.get(t)||null}const V=()=>typeof CSS<"u"&&"highlights"in CSS&&typeof window.Highlight<"u";class X{element;overlay=null;shadowRoot=null;tooltipContainer=null;tooltipShadowRoot=null;resizeObserver=null;tooltip=null;currentMatches=[];ignoredMatches=new Set;personalDictionary=new Set;callbacks=null;boundHideTooltip;useCustomHighlights=!1;highlightStyleSheet=null;constructor(t){this.element=t,this.boundHideTooltip=this.handleOutsideClick.bind(this);const e=!(t instanceof HTMLInputElement||t instanceof HTMLTextAreaElement);this.useCustomHighlights=e&&V(),this.useCustomHighlights&&(console.log("[AutoCorrect] Using CSS Custom Highlights API (modern path)"),this.setupCustomHighlightStyles())}setupCustomHighlightStyles(){this.highlightStyleSheet=new CSSStyleSheet,this.highlightStyleSheet.replaceSync(`
      ::highlight(autocorrect-spelling) {
        text-decoration: underline wavy #EF4444;
        text-decoration-skip-ink: none;
        text-underline-offset: 2px;
      }
      ::highlight(autocorrect-grammar) {
        text-decoration: underline wavy #F59E0B;
        text-decoration-skip-ink: none;
        text-underline-offset: 2px;
      }
      ::highlight(autocorrect-style) {
        text-decoration: underline wavy #3B82F6;
        text-decoration-skip-ink: none;
        text-underline-offset: 2px;
      }
      @media (prefers-color-scheme: dark) {
        ::highlight(autocorrect-spelling) {
          text-decoration-color: #F87171;
        }
        ::highlight(autocorrect-grammar) {
          text-decoration-color: #FBBF24;
        }
        ::highlight(autocorrect-style) {
          text-decoration-color: #60A5FA;
        }
      }
    `),document.adoptedStyleSheets=[...document.adoptedStyleSheets,this.highlightStyleSheet]}init(t){this.callbacks=t,this.createOverlay(),this.setupObservers()}setDictionary(t){this.personalDictionary=new Set(t.map(e=>e.toLowerCase()))}createOverlay(){this.overlay=document.createElement("div"),this.overlay.className="autocorrect-overlay",this.overlay.style.cssText=`
      position: absolute;
      pointer-events: none;
      overflow: visible;
      z-index: 2147483646;
    `,this.shadowRoot=this.overlay.attachShadow({mode:"open"});const t=document.createElement("style");t.textContent=`
      @import url('https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600&display=swap');

      :host {
        all: initial;
        font-family: 'DM Sans', -apple-system, BlinkMacSystemFont, sans-serif;
        /* Light mode - warm editorial palette */
        --ac-cream: #FAF8F5;
        --ac-paper: #FFFFFF;
        --ac-ink: #1A1612;
        --ac-ink-soft: #4A4540;
        --ac-ink-muted: #8A857D;
        --ac-border: #E8E4DE;
        --ac-border-soft: #F2EFEA;
        --ac-coral: #E85D4C;
        --ac-coral-soft: #FEF2F0;
        --ac-amber: #D4940A;
        --ac-amber-soft: #FDF8EC;
        --ac-indigo: #5B6AD0;
        --ac-indigo-soft: #F3F4FC;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          --ac-cream: #1A1612;
          --ac-paper: #242019;
          --ac-ink: #FAF8F5;
          --ac-ink-soft: #C9C4BC;
          --ac-ink-muted: #7A756D;
          --ac-border: #3A352D;
          --ac-border-soft: #2A2620;
          --ac-coral: #F08070;
          --ac-coral-soft: #2D201E;
          --ac-amber: #E8A820;
          --ac-amber-soft: #2A2418;
          --ac-indigo: #7B8AE0;
          --ac-indigo-soft: #1E2030;
        }
      }

      /* Error underlines - now clickable */
      .error-highlight {
        position: absolute;
        background: transparent;
        cursor: pointer;
        pointer-events: auto;
        border-radius: 2px;
        animation: highlightFadeIn 0.2s ease-out;
      }
      @keyframes highlightFadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
      }
      .error-highlight:hover {
        background: rgba(232, 93, 76, 0.08);
      }
      .error-highlight::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 3px;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23E85D4C' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
        background-repeat: repeat-x;
        background-position: bottom;
        background-size: 8px 4px;
      }
      .error-spelling::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23E85D4C' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
      }
      @media (prefers-color-scheme: dark) {
        .error-spelling::after {
          background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23F08070' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
        }
      }
      .error-grammar::after {
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23D4940A' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
      }
      @media (prefers-color-scheme: dark) {
        .error-grammar::after {
          background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='4' viewBox='0 0 8 4'%3E%3Cpath d='M0 3 Q2 0 4 3 Q6 6 8 3' stroke='%23E8A820' fill='none' stroke-width='1.5'/%3E%3C/svg%3E");
        }
      }
      .error-grammar:hover {
        background: rgba(212, 148, 10, 0.08);
      }

      /* Tooltip - refined editorial design */
      .tooltip {
        position: fixed;
        background: var(--ac-paper);
        border-radius: 16px;
        box-shadow: 0 8px 32px rgba(26, 22, 18, 0.12), 0 0 0 1px var(--ac-border-soft);
        padding: 0;
        min-width: 260px;
        max-width: 340px;
        z-index: 2147483647;
        animation: tooltipIn 0.2s cubic-bezier(0.22, 1, 0.36, 1);
        overflow: hidden;
        pointer-events: auto;
      }
      @media (prefers-color-scheme: dark) {
        .tooltip {
          box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4), 0 0 0 1px var(--ac-border);
        }
      }
      @keyframes tooltipIn {
        from {
          opacity: 0;
          transform: translateY(-8px) scale(0.96);
        }
        to {
          opacity: 1;
          transform: translateY(0) scale(1);
        }
      }

      .tooltip-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 14px;
        border-bottom: 1px solid var(--ac-border-soft);
      }

      .category-badge {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 4px 10px;
        border-radius: 20px;
        font-weight: 500;
        font-size: 12px;
      }
      .category-badge.spelling {
        background: var(--ac-coral-soft);
        color: var(--ac-coral);
      }
      .category-badge.grammar {
        background: var(--ac-amber-soft);
        color: var(--ac-amber);
      }
      .category-badge.style {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
      }
      .category-badge .dot {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: currentColor;
      }

      .tooltip-close {
        width: 28px;
        height: 28px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: transparent;
        color: var(--ac-ink-muted);
        cursor: pointer;
        border-radius: 8px;
        transition: all 0.15s ease;
      }
      .tooltip-close:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
      }

      .tooltip-body {
        padding: 14px;
      }
      .tooltip-message {
        color: var(--ac-ink-soft);
        font-size: 13px;
        line-height: 1.55;
        margin-bottom: 14px;
      }

      .tooltip-suggestions {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        margin-bottom: 12px;
      }
      .suggestion-btn {
        padding: 8px 16px;
        background: var(--ac-ink);
        color: var(--ac-paper);
        border: none;
        border-radius: 10px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .suggestion-btn:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(26, 22, 18, 0.15);
      }
      .suggestion-btn:active {
        transform: translateY(0);
      }
      .suggestion-btn.secondary {
        background: transparent;
        color: var(--ac-ink);
        border: 1px solid var(--ac-border);
      }
      .suggestion-btn.secondary:hover {
        background: var(--ac-border-soft);
        box-shadow: none;
        transform: none;
      }

      .tooltip-actions {
        display: flex;
        gap: 8px;
        padding-top: 12px;
        border-top: 1px solid var(--ac-border-soft);
      }
      .action-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        padding: 8px 12px;
        background: transparent;
        color: var(--ac-ink-muted);
        border: 1px solid var(--ac-border);
        border-radius: 8px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .action-btn:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
        border-color: var(--ac-border);
      }
      .action-btn.dictionary:hover {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
        border-color: var(--ac-indigo-soft);
      }
      .action-btn svg {
        width: 14px;
        height: 14px;
      }
    `,this.shadowRoot.appendChild(t),document.body.appendChild(this.overlay),this.updatePosition(),this.tooltipContainer=document.createElement("autocorrect-tooltip-portal"),this.tooltipContainer.style.cssText=`
      display: block !important;
      position: fixed !important;
      top: 0 !important;
      left: 0 !important;
      width: 0 !important;
      height: 0 !important;
      overflow: visible !important;
      z-index: 2147483647 !important;
      pointer-events: none !important;
    `,this.tooltipShadowRoot=this.tooltipContainer.attachShadow({mode:"open"});const e=document.createElement("style");e.textContent=this.getTooltipStyles(),this.tooltipShadowRoot.appendChild(e),document.body.appendChild(this.tooltipContainer)}getTooltipStyles(){return`
      @import url('https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600&display=swap');

      :host {
        all: initial;
        font-family: 'DM Sans', -apple-system, BlinkMacSystemFont, sans-serif;
        /* Light mode - warm editorial palette */
        --ac-cream: #FAF8F5;
        --ac-paper: #FFFFFF;
        --ac-ink: #1A1612;
        --ac-ink-soft: #4A4540;
        --ac-ink-muted: #8A857D;
        --ac-border: #E8E4DE;
        --ac-border-soft: #F2EFEA;
        --ac-coral: #E85D4C;
        --ac-coral-soft: #FEF2F0;
        --ac-amber: #D4940A;
        --ac-amber-soft: #FDF8EC;
        --ac-indigo: #5B6AD0;
        --ac-indigo-soft: #F3F4FC;
      }

      @media (prefers-color-scheme: dark) {
        :host {
          --ac-cream: #1A1612;
          --ac-paper: #242019;
          --ac-ink: #FAF8F5;
          --ac-ink-soft: #C9C4BC;
          --ac-ink-muted: #7A756D;
          --ac-border: #3A352D;
          --ac-border-soft: #2A2620;
          --ac-coral: #F08070;
          --ac-coral-soft: #2D201E;
          --ac-amber: #E8A820;
          --ac-amber-soft: #2A2418;
          --ac-indigo: #7B8AE0;
          --ac-indigo-soft: #1E2030;
        }
      }

      /* Tooltip - refined editorial design */
      .tooltip {
        position: fixed;
        background: var(--ac-paper);
        border-radius: 16px;
        box-shadow: 0 8px 32px rgba(26, 22, 18, 0.12), 0 0 0 1px var(--ac-border-soft);
        padding: 0;
        min-width: 260px;
        max-width: 340px;
        z-index: 2147483647;
        animation: tooltipIn 0.2s cubic-bezier(0.22, 1, 0.36, 1);
        overflow: hidden;
        pointer-events: auto;
      }
      @media (prefers-color-scheme: dark) {
        .tooltip {
          box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4), 0 0 0 1px var(--ac-border);
        }
      }
      @keyframes tooltipIn {
        from {
          opacity: 0;
          transform: translateY(-8px) scale(0.96);
        }
        to {
          opacity: 1;
          transform: translateY(0) scale(1);
        }
      }

      .tooltip-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 14px;
        border-bottom: 1px solid var(--ac-border-soft);
      }

      .category-badge {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 4px 10px;
        border-radius: 20px;
        font-weight: 500;
        font-size: 12px;
      }
      .category-badge.spelling {
        background: var(--ac-coral-soft);
        color: var(--ac-coral);
      }
      .category-badge.grammar {
        background: var(--ac-amber-soft);
        color: var(--ac-amber);
      }
      .category-badge.style {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
      }
      .category-badge .dot {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: currentColor;
      }

      .tooltip-close {
        width: 28px;
        height: 28px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: transparent;
        color: var(--ac-ink-muted);
        cursor: pointer;
        border-radius: 8px;
        transition: all 0.15s ease;
      }
      .tooltip-close:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
      }

      .tooltip-body {
        padding: 14px;
      }
      .tooltip-message {
        color: var(--ac-ink-soft);
        font-size: 13px;
        line-height: 1.55;
        margin-bottom: 14px;
      }

      .tooltip-suggestions {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        margin-bottom: 12px;
      }
      .suggestion-btn {
        padding: 8px 16px;
        background: var(--ac-ink);
        color: var(--ac-paper);
        border: none;
        border-radius: 10px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .suggestion-btn:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(26, 22, 18, 0.15);
      }
      .suggestion-btn:active {
        transform: translateY(0);
      }
      .suggestion-btn.secondary {
        background: transparent;
        color: var(--ac-ink);
        border: 1px solid var(--ac-border);
      }
      .suggestion-btn.secondary:hover {
        background: var(--ac-border-soft);
        box-shadow: none;
        transform: none;
      }

      .tooltip-actions {
        display: flex;
        gap: 8px;
        padding-top: 12px;
        border-top: 1px solid var(--ac-border-soft);
      }
      .action-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        padding: 8px 12px;
        background: transparent;
        color: var(--ac-ink-muted);
        border: 1px solid var(--ac-border);
        border-radius: 8px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.15s ease;
        pointer-events: auto;
      }
      .action-btn:hover {
        background: var(--ac-border-soft);
        color: var(--ac-ink-soft);
        border-color: var(--ac-border);
      }
      .action-btn.dictionary:hover {
        background: var(--ac-indigo-soft);
        color: var(--ac-indigo);
        border-color: var(--ac-indigo-soft);
      }
      .action-btn svg {
        width: 14px;
        height: 14px;
      }
    `}setupObservers(){this.resizeObserver=new ResizeObserver(()=>{this.updatePosition()}),this.resizeObserver.observe(this.element),this.element.addEventListener("scroll",()=>{this.updatePosition(),this.hideTooltip()}),window.addEventListener("scroll",()=>{this.updatePosition(),this.hideTooltip()},!0),window.addEventListener("resize",()=>{this.updatePosition(),this.hideTooltip()}),this.element.addEventListener("input",()=>{this.hideTooltip()})}updatePosition(){if(!this.overlay)return;const t=this.element.getBoundingClientRect(),e=window.scrollX,o=window.scrollY;this.overlay.style.left=`${t.left+e}px`,this.overlay.style.top=`${t.top+o}px`,this.overlay.style.width=`${t.width}px`,this.overlay.style.height=`${t.height}px`}render(t,e){this.currentMatches=t,this.hideTooltip();const o=t.filter(r=>{const c=`${r.offset}-${r.length}-${r.rule.id}`;if(this.ignoredMatches.has(c))return!1;const d=e.substring(r.offset,r.offset+r.length).toLowerCase();return!this.personalDictionary.has(d)});if(console.log("[AutoCorrect] Rendering",o.length,"matches (useCustomHighlights:",this.useCustomHighlights,")"),o.length===0){this.clearHighlights();return}if(this.useCustomHighlights){this.renderWithCustomHighlights(o,e);return}if(!this.shadowRoot)return;this.shadowRoot.querySelectorAll(".error-highlight").forEach(r=>r.remove());const i=this.getVisibleTextRange(e),a=o.filter(r=>{const c=r.offset+r.length,d=Math.max(0,i.startOffset-500),g=i.endOffset+500;return c>d&&r.offset<g});console.log("[AutoCorrect] Visible matches:",a.length,"range:",i);const l=this.calculatePositions(a,e);console.log("[AutoCorrect] Positions calculated:",l);let h=0;l.forEach(r=>{const c=a[r.matchIndex];if(!c)return;const d=this.currentMatches.indexOf(c);if(r.y<-50||r.y>this.element.clientHeight+50)return;const g=document.createElement("span");g.className=`error-highlight ${this.getErrorClass(c)}`,g.style.left=`${r.x}px`,g.style.top=`${r.y}px`,g.style.width=`${r.width}px`,g.style.height=`${r.height}px`,g.dataset.matchIndex=String(d),g.addEventListener("click",p=>{p.preventDefault(),p.stopPropagation(),this.showTooltip(c,p.clientX,p.clientY,r)}),this.shadowRoot.appendChild(g),h++}),console.log("[AutoCorrect] Rendered",h,"underlines (multi-rect support enabled)")}renderWithCustomHighlights(t,e){this.clearHighlights();const o=this.element,s=R(o),i=[],a=[],l=[];t.forEach(c=>{try{const d=w(s,c.offset);if(!d)return;const g=w(s,c.offset+c.length-1);if(!g)return;const p=document.createRange();p.setStart(d.node,d.offset),p.setEnd(g.node,Math.min(g.offset+1,g.node.length));const b=c.rule.category.id.toUpperCase();b.includes("TYPO")||b.includes("SPELL")?i.push(p):b.includes("GRAMMAR")?a.push(p):l.push(p)}catch(d){console.log("[AutoCorrect] CSS Highlights range error:",d)}});const h=CSS.highlights,r=window.Highlight;i.length>0&&h.set("autocorrect-spelling",new r(...i)),a.length>0&&h.set("autocorrect-grammar",new r(...a)),l.length>0&&h.set("autocorrect-style",new r(...l)),console.log("[AutoCorrect] CSS Custom Highlights rendered:",{spelling:i.length,grammar:a.length,style:l.length}),this.renderClickTargets(t,e)}renderClickTargets(t,e){if(!this.shadowRoot)return;this.shadowRoot.querySelectorAll(".click-target").forEach(i=>i.remove()),this.calculatePositions(t,e).forEach(i=>{const a=t[i.matchIndex];if(!a)return;const l=this.currentMatches.indexOf(a),h=document.createElement("span");h.className="click-target",h.style.cssText=`
        position: absolute;
        left: ${i.x}px;
        top: ${i.y}px;
        width: ${i.width}px;
        height: ${i.height}px;
        cursor: pointer;
        pointer-events: auto;
        background: transparent;
      `,h.dataset.matchIndex=String(l),h.addEventListener("click",r=>{r.preventDefault(),r.stopPropagation(),this.showTooltip(a,r.clientX,r.clientY,i)}),this.shadowRoot.appendChild(h)})}clearHighlights(){if(this.useCustomHighlights){const t=CSS.highlights;t.delete("autocorrect-spelling"),t.delete("autocorrect-grammar"),t.delete("autocorrect-style")}this.shadowRoot&&this.shadowRoot.querySelectorAll(".error-highlight, .click-target").forEach(e=>e.remove())}getVisibleTextRange(t){const e=this.element;if(e instanceof HTMLInputElement)return{startOffset:0,endOffset:t.length};if(e instanceof HTMLTextAreaElement){const o=window.getComputedStyle(e),s=parseFloat(o.lineHeight)||parseFloat(o.fontSize)*1.2,i=e.scrollTop,a=e.clientHeight,l=Math.max(0,Math.floor(i/s)-2),h=Math.ceil((i+a)/s)+2,r=t.split(`
`);let c=0,d=t.length;for(let p=0;p<l&&p<r.length;p++)c+=r[p].length+1;let g=0;for(let p=0;p<=h&&p<r.length;p++)g+=r[p].length+1;return d=Math.min(g,t.length),{startOffset:c,endOffset:d}}return{startOffset:0,endOffset:t.length}}getErrorClass(t){const e=t.rule.category.id.toUpperCase();return e.includes("TYPO")||e.includes("SPELL")?"error-spelling":(e.includes("GRAMMAR"),"error-grammar")}getCategoryInfo(t){const e=t.rule.category.id.toUpperCase();return e.includes("TYPO")||e.includes("SPELL")?{name:"Orthographe",class:"spelling"}:e.includes("GRAMMAR")?{name:"Grammaire",class:"grammar"}:{name:"Style",class:"style"}}showTooltip(t,e,o,s){if(this.hideTooltip(),!this.tooltipShadowRoot)return;const i=this.getCategoryInfo(t);this.tooltip=document.createElement("div"),this.tooltip.className="tooltip";const a=window.innerHeight,l=150,h=a-o-20;let r=o+10;h<l&&o>l&&(r=o-l-10),this.tooltip.style.left=`${Math.min(e-20,window.innerWidth-340)}px`,this.tooltip.style.top=`${r}px`;const c=this.getElementText().substring(t.offset,t.offset+t.length);this.tooltip.innerHTML=`
      <div class="tooltip-header">
        <div class="category-badge ${i.class}">
          <span class="dot"></span>
          <span>${i.name}</span>
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
          ${t.replacements.slice(0,3).map((d,g)=>`<button class="suggestion-btn${g>0?" secondary":""}" data-replacement="${this.escapeHtml(d.value)}">${this.escapeHtml(d.value)}</button>`).join("")}
        </div>
        <div class="tooltip-actions">
          <button class="action-btn ignore-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18.36 6.64A9 9 0 1 1 5.64 6.64 9 9 0 0 1 18.36 6.64Z"/>
              <path d="M6 6l12 12"/>
            </svg>
            Ignorer
          </button>
          <button class="action-btn dictionary dictionary-btn">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
            </svg>
            Dictionnaire
          </button>
        </div>
      </div>
    `,this.tooltip.querySelector(".tooltip-close")?.addEventListener("click",()=>{this.hideTooltip()}),this.tooltip.querySelectorAll(".suggestion-btn").forEach(d=>{d.addEventListener("click",g=>{g.stopPropagation();const p=g.target.dataset.replacement||"";console.log("[AutoCorrect] Suggestion clicked:",p),this.callbacks?.onReplace(t,p),this.hideTooltip()})}),this.tooltip.querySelector(".ignore-btn")?.addEventListener("click",()=>{const d=`${t.offset}-${t.length}-${t.rule.id}`;this.ignoredMatches.add(d),this.callbacks?.onIgnore(t),this.hideTooltip(),this.render(this.currentMatches,this.getElementText())}),this.tooltip.querySelector(".dictionary-btn")?.addEventListener("click",()=>{this.personalDictionary.add(c.toLowerCase()),this.callbacks?.onIgnore(t),this.hideTooltip(),this.render(this.currentMatches,this.getElementText())}),this.tooltipShadowRoot.appendChild(this.tooltip),setTimeout(()=>{document.addEventListener("click",this.boundHideTooltip,!1)},10)}handleOutsideClick(t){const e=t.composedPath();this.tooltip&&!e.includes(this.tooltip)&&this.hideTooltip()}hideTooltip(){this.tooltip&&(this.tooltip.remove(),this.tooltip=null,document.removeEventListener("click",this.boundHideTooltip,!1))}escapeHtml(t){const e=document.createElement("div");return e.textContent=t,e.innerHTML}getElementText(){return this.element instanceof HTMLInputElement||this.element instanceof HTMLTextAreaElement?this.element.value:this.element.innerText||""}calculatePositions(t,e){const o=this.element instanceof HTMLInputElement,s=this.element instanceof HTMLTextAreaElement;return o||s?this.calculateInputPositions(t,e):this.calculateContentEditablePositions(t,e)}calculateInputPositions(t,e){const o=this.element,s=window.getComputedStyle(o),i=document.createElement("div");i.style.cssText=`
      position: absolute;
      top: -9999px;
      left: -9999px;
      visibility: hidden;
      white-space: pre-wrap;
      word-wrap: break-word;
      overflow-wrap: break-word;
      font-family: ${s.fontFamily};
      font-size: ${s.fontSize};
      font-weight: ${s.fontWeight};
      font-style: ${s.fontStyle};
      letter-spacing: ${s.letterSpacing};
      word-spacing: ${s.wordSpacing};
      line-height: ${s.lineHeight};
      text-transform: ${s.textTransform};
      padding: ${s.padding};
      border: ${s.borderWidth} solid transparent;
      box-sizing: border-box;
      width: ${o.offsetWidth}px;
    `,document.body.appendChild(i);const a=[],l=parseFloat(s.paddingTop)||0,h=parseFloat(s.borderTopWidth)||0,r=parseFloat(s.lineHeight)||parseFloat(s.fontSize)*1.2,c=o.scrollLeft||0,d=o.scrollTop||0;return t.forEach((g,p)=>{const b=e.substring(0,g.offset),N=e.substring(g.offset,g.offset+g.length);i.innerHTML="";const A=b.split(`
`),M=A.length-1,z=A[M],F=document.createElement("span");F.textContent=z,i.appendChild(F);const y=document.createElement("span");y.textContent=N,i.appendChild(y);const L=y.getBoundingClientRect(),B=i.getBoundingClientRect(),U=L.left-B.left-c,q=L.width||y.offsetWidth||10,G=l+h+M*r-d;a.push({x:Math.max(0,U),y:G,width:Math.max(q,10),height:r,matchIndex:p})}),document.body.removeChild(i),a}calculateContentEditablePositions(t,e){const o=[],s=this.element,i=s.getBoundingClientRect(),a=R(s);return t.forEach((l,h)=>{try{const r=w(a,l.offset);if(!r){console.log("[AutoCorrect] Could not find start position for offset:",l.offset);return}const c=w(a,l.offset+l.length-1);if(!c){console.log("[AutoCorrect] Could not find end position for offset:",l.offset+l.length-1);return}const d=document.createRange();d.setStart(r.node,r.offset),d.setEnd(c.node,Math.min(c.offset+1,c.node.length));const g=d.getClientRects();Array.from(g).forEach(p=>{o.push({x:p.left-i.left+s.scrollLeft,y:p.top-i.top+s.scrollTop,width:Math.max(p.width,10),height:p.height,matchIndex:h})})}catch(r){console.log("[AutoCorrect] Position calculation error:",r)}}),o}destroy(){if(this.hideTooltip(),this.clearHighlights(),this.resizeObserver&&this.resizeObserver.disconnect(),this.overlay&&this.overlay.remove(),this.tooltipContainer&&(this.tooltipContainer.remove(),this.tooltipContainer=null,this.tooltipShadowRoot=null),this.highlightStyleSheet){const t=document.adoptedStyleSheets.indexOf(this.highlightStyleSheet);t!==-1&&(document.adoptedStyleSheets=document.adoptedStyleSheets.filter((e,o)=>o!==t)),this.highlightStyleSheet=null}}}const H=400,v=new WeakMap;let m=null,u=null;function I(n){m=n,n.enabled||document.querySelectorAll(".autocorrect-overlay").forEach(t=>t.remove()),document.querySelectorAll('input, textarea, [contenteditable]:not([contenteditable="false"])').forEach(t=>{const e=v.get(t);e&&e.renderer.setDictionary(n.personalDictionary||[])})}function E(n){if(n instanceof HTMLInputElement){const t=n.type.toLowerCase();return!(!["text","search","email","url","tel",""].includes(t)||n.offsetWidth<100)}if(n instanceof HTMLTextAreaElement)return!0;if(n instanceof HTMLElement&&n.isContentEditable){if(n.offsetWidth<100||n.offsetHeight<30)return!1;const t=n.getAttribute("role");return!(t&&["button","menuitem","option","tab"].includes(t))}return!1}function f(n){let t;return n instanceof HTMLInputElement||n instanceof HTMLTextAreaElement?t=n.value:t=n.innerText||"",t.normalize("NFC")}function J(n,t=500){const e=f(n);if(e.length<=t)return{text:e,offset:0};let o=0;if(n instanceof HTMLInputElement||n instanceof HTMLTextAreaElement)o=n.selectionStart||0;else{const r=window.getSelection();if(r&&r.rangeCount>0){const c=r.getRangeAt(0),d=document.createRange();d.selectNodeContents(n),d.setEnd(c.startContainer,c.startOffset),o=d.toString().length}}const s=e.lastIndexOf(`

`,o),i=e.indexOf(`

`,o);let a=s===-1?0:s+2,l=i===-1?e.length:i;const h=Math.floor(t/2);for(l-a<t&&(a=Math.max(0,o-h),l=Math.min(e.length,o+h));a>0&&e[a-1]!==" "&&e[a-1]!==`
`;)a--;for(;l<e.length&&e[l]!==" "&&e[l]!==`
`;)l++;return{text:e.substring(a,l),offset:a}}function O(n,t,e,o){const s=f(n),i=s.substring(t,t+e);if(console.log("[AutoCorrect] setTextContent called:",{offset:t,length:e,replacement:o,elementType:n.tagName,matchedText:`"${i}"`,contextBefore:`"${s.substring(Math.max(0,t-5),t)}"`,contextAfter:`"${s.substring(t+e,t+e+5)}"`}),i.length!==e&&console.warn("[AutoCorrect] WARNING: Matched text length mismatch!",{expected:e,actual:i.length,matchedText:`"${i}"`}),n instanceof HTMLInputElement||n instanceof HTMLTextAreaElement){const a=n.value;console.log("[AutoCorrect] Input/Textarea replacement:",{textLength:a.length,beforeOffset:t});const l=a.substring(0,t),h=a.substring(t+e);n.value=l+o+h;const r=t+o.length;n.setSelectionRange(r,r),n.dispatchEvent(new Event("input",{bubbles:!0})),console.log("[AutoCorrect] Replacement done for input/textarea")}else{console.log("[AutoCorrect] Contenteditable replacement at offset:",t,"length:",e);const a=K(n,t,e);if(!a){console.warn("[AutoCorrect] Could not create range for replacement");return}const l=a.toString();if(console.log("[AutoCorrect] Range created:",{rangeText:`"${l}"`,expectedText:`"${i}"`,matches:l===i}),l!==i&&console.warn("[AutoCorrect] WARNING: Range text mismatch! Expected:",`"${i}"`,"Got:",`"${l}"`),n.classList.contains("ck-editor__editable")||n.classList.contains("ck-content")){console.log("[AutoCorrect] CKEditor detected, using paste simulation");const r=window.getSelection();r&&(r.removeAllRanges(),r.addRange(a),setTimeout(async()=>{try{r.removeAllRanges(),r.addRange(a),await navigator.clipboard.writeText(o),console.log("[AutoCorrect] Clipboard written, triggering paste");const c=new ClipboardEvent("paste",{bubbles:!0,cancelable:!0,clipboardData:new DataTransfer});c.clipboardData?.setData("text/plain",o);const d=n.dispatchEvent(c);console.log("[AutoCorrect] Paste event dispatched, handled:",d),(!d||c.defaultPrevented)&&document.execCommand("insertText",!1,o)}catch(c){console.error("[AutoCorrect] Paste simulation failed:",c),document.execCommand("insertText",!1,o)}},10))}else n.focus(),setTimeout(()=>{try{const r=window.getSelection();if(r){r.removeAllRanges(),r.addRange(a);const c=r.toString();console.log("[AutoCorrect] Selection set:",{selectedText:`"${c}"`,expectedText:`"${i}"`,matches:c===i}),document.execCommand("insertText",!1,o)?console.log("[AutoCorrect] Replacement done via execCommand"):(console.log("[AutoCorrect] execCommand failed, trying delete + insertText"),document.execCommand("delete",!1),document.execCommand("insertText",!1,o))}}catch(r){console.error("[AutoCorrect] Error during replacement:",r)}},10);n.dispatchEvent(new InputEvent("input",{bubbles:!0,inputType:"insertText",data:o}))}}function P(n,t,e){e.debounceTimer!==null&&clearTimeout(e.debounceTimer),e.debounceTimer=window.setTimeout(()=>{e.debounceTimer=null,n()},t)}async function T(n){if(!m?.enabled||!m?.apiUrl){console.log("[AutoCorrect] Disabled or no API URL");return}const t=f(n.element);if(t===n.lastText){console.log("[AutoCorrect] Text unchanged, skipping");return}if(n.lastText=t,t.trim().length<3){n.currentMatches=[],n.renderer.render([],t);return}const{text:e,offset:o}=J(n.element,500);console.log("[AutoCorrect] Checking text:",e.substring(0,50),"... (",e.length,"of",t.length,"chars, offset:",o,")"),console.log("[AutoCorrect] Calling API...");const i=(await Q(e,m.language,m.apiUrl)).map(a=>({...a,offset:a.offset+o}));console.log("[AutoCorrect] Got",i.length,"matches"),n.currentMatches=i,n.renderer.render(i,t)}function x(n){if(v.has(n))return;console.log("[AutoCorrect] Attaching to field:",n.tagName,n.className?.substring?.(0,50));const t=new X(n),e={element:n,renderer:t,debounceTimer:null,lastText:"",currentMatches:[]};t.init({onReplace:(o,s)=>{O(n,o.offset,o.length,s)},onIgnore:o=>{const i=f(n).substring(o.offset,o.offset+o.length);W(i)}}),m?.personalDictionary&&t.setDictionary(m.personalDictionary),v.set(n,e),n.addEventListener("input",()=>{P(()=>T(e),H,e)}),n.addEventListener("focus",()=>{u=e,f(n).trim().length>=3&&P(()=>T(e),H,e)}),n.addEventListener("blur",()=>{}),document.activeElement===n&&(u=e,f(n).trim().length>=3&&setTimeout(()=>T(e),500))}function k(n){const t=v.get(n);t&&(t.debounceTimer!==null&&clearTimeout(t.debounceTimer),t.renderer.destroy(),v.delete(n))}function S(){const n=document.querySelectorAll('input[type="text"], input[type="search"], input[type="email"], input[type="url"], input[type="tel"], input:not([type])'),t=document.querySelectorAll("textarea"),e=document.querySelectorAll('[contenteditable]:not([contenteditable="false"])');console.log("[AutoCorrect] Scan found:",n.length,"inputs,",t.length,"textareas,",e.length,"contenteditables"),n.forEach(o=>{E(o)&&x(o)}),t.forEach(o=>{x(o)}),e.forEach(o=>{o instanceof HTMLElement&&x(o)})}function D(){S(),new MutationObserver(t=>{t.forEach(e=>{if(e.type==="attributes"&&e.attributeName==="contenteditable"){const o=e.target;o instanceof HTMLElement&&E(o)&&(console.log("[AutoCorrect] Contenteditable attribute changed on:",o.tagName,o.className?.substring?.(0,50)),x(o))}e.addedNodes.forEach(o=>{o instanceof Element&&(E(o)&&x(o),o.querySelectorAll('input, textarea, [contenteditable]:not([contenteditable="false"])').forEach(s=>{E(s)&&x(s)}))}),e.removedNodes.forEach(o=>{o instanceof Element&&(k(o),o.querySelectorAll("input, textarea, [contenteditable]").forEach(k))})})}).observe(document.body,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["contenteditable"]}),setTimeout(S,2e3),setTimeout(S,5e3),Z()}function Z(){chrome.runtime.onMessage.addListener((n,t,e)=>{if(n.type==="GET_MATCHES"){const o={type:"MATCHES_RESPONSE",matches:u?.currentMatches||[],textLength:u?f(u.element).length:0,fieldInfo:u?{tagName:u.element.tagName.toLowerCase(),hasContent:f(u.element).trim().length>0}:null};return e(o),!0}if(n.type==="APPLY_SUGGESTION"){const o=n;if(u&&u.currentMatches[o.matchIndex]){const s=u.currentMatches[o.matchIndex];O(u.element,s.offset,s.length,o.replacement),e({type:"SUGGESTION_APPLIED",success:!0})}else e({type:"SUGGESTION_APPLIED",success:!1});return!0}return!1})}function tt(){document.querySelectorAll('input, textarea, [contenteditable="true"]').forEach(k)}async function $(){const n=await Y();I(n),n.enabled&&D(),_(t=>{I(t),t.enabled?D():tt()})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",$):$();
