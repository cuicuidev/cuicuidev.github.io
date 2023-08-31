//Social Media link with respective icons

export class Socials extends HTMLAnchorElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.render();
  }

  render() {
    this.shadowRoot.innerHTML /*html*/`
        <style> 
        svg {fill: white; stroke: white;}
        </style>
        
        <ul>
        <li><a><svg></svg></a></li>
        </ul>
        `;
  }
}

customElements.define("social-links", Socials);
