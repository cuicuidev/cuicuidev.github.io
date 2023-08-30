//Article Card containing its respective project

export class Projects extends HTMLElement {
    constructor() {
      super();
      this.attachShadow({ mode: "open" });
      this.render();
    }
  
    render() {
      this.shadowRoot.innerHTML /*html*/`

        <style>
        </style>
        
        <a>
        <article>
        <img />
        <h3>cuicui</h3>
        <p></p>
        </article>
        </a>
        `;
    }
  }
  
  customElements.define("project-card", Projects);