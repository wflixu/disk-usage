import { createApp } from "vue";
import App from "./App.vue";

import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
import { setupUILib } from "./shared/uilib";
import { setupEcharts } from "./shared/setup-echarts";
import "./style/style.css"

const app = createApp(App)
app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
});
app.use(setupUILib)
app.use(setupEcharts)
app.mount("#app");
