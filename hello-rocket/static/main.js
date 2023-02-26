customElements.define(
  "delodge-location",
  class extends HTMLElement {
    constructor() {
      super();
      let template = document.getElementById("delodge-location");
      let templateContent = template.content;

      const shadowRoot = this.attachShadow({ mode: "open" });
      shadowRoot.appendChild(templateContent.cloneNode(true));
    }
  }
);

const html = (strings, ...values) => String.raw({ raw: strings }, ...values);
const DeLogdeLocation = (name, description) => html`<delodge-location>
  <span slot="name">${name}</span>
  <span slot="description">${description}</span>
</delodge-location>`;

fetch("/api/locations")
  .then((response) => response.json())
  .then((data) =>
    (data || []).forEach((location) =>
      document.body.insertAdjacentHTML(
        "beforeend",
        DeLogdeLocation(location?.name, location?.description)
      )
    )
  );
