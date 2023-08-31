//Testimonials Card for the Testimonials Carousel section

export class Testimonials extends HTMLElement {
    constructor() {
      super();
      this.attachShadow({ mode: "open" });
      this.render();
    }
  
    render() {
      this.shadowRoot.innerHTML /*html*/`

        <style>
        </style>
        
        <div>
        <img />
        <p></p>
        <p>
        <a></a>
        <a></a>
        </p>
        </div>
        `;
    }
  }
  
  customElements.define("testimonial-card", Testimonials);