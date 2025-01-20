// echarts-init.ts

import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { PieChart, LinesChart, LineChart, TreemapChart } from "echarts/charts";
import {
    TitleComponent,
    TooltipComponent,
    LegendComponent,
    GridComponent,
    DataZoomComponent,
    DatasetComponent,
} from "echarts/components";
import { THEME_KEY } from "vue-echarts";
import { type App, type Plugin } from "vue";
import VChart from "vue-echarts";

export const setupEcharts: Plugin = (app: App, ...options: any[]) => {
    use([
        CanvasRenderer,
        DatasetComponent,
        PieChart,
        LinesChart,
        LineChart,
        TreemapChart,
        TitleComponent,
        TooltipComponent,
        LegendComponent,
        GridComponent,
        DataZoomComponent,
    ]);

    app.provide(THEME_KEY, "");
    // 把echart 组成到
    app.component("v-chart", VChart);
};