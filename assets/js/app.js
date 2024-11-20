import lodash from "lodash";
window._ = lodash;

import "../css/app.css";
import { createApp } from "vue";
import App from "./App.vue"

const app = createApp(App);
app.mount("#app");
