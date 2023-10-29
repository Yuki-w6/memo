import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import { createPinia } from "pinia";
import msg from "@/assets/msg/msg.json.js";
import "./style.css";
import App from "./App.vue";

const app = createApp(App);

const messages = {
    msg,
};

const i18n = createI18n({
    legacy: false,
    locale: "msg",
    messages,
});

app.use(createPinia());
app.use(i18n);
app.mount("#app");
