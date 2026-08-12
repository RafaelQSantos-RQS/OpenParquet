import { createApp } from "vue";
import { createPinia } from "pinia";
import { createVuetify } from "vuetify";
import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";
import App from "./App.vue";

// Dark-only theme (light was removed).
const vuetify = createVuetify({
  theme: {
    defaultTheme: "dark",
  },
});

const app = createApp(App);

app.use(createPinia());
app.use(vuetify);

app.mount("#app");
