import { createApp } from "vue";
import "./style.css";
import App from "./App";
import router from "./router";
import { i18n } from './hooks/i18n'

const app = createApp(App);
app.use(router);
app.use(i18n);
app.mount("#app");
