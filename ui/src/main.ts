import { createHead } from "@unhead/vue/client";
import { createApp } from "vue";
// @ts-ignore
import App from "./App.vue";

import "./style.css";

const head = createHead();
const app = createApp(App);
app.use(head);

app.mount("#app");
