<template>
    <div class="box">
        <v-chart class="chart" :option="chartOptions" />
    </div>
</template>

<script setup lang="ts">
import diskData from './data.json'
import { onMounted, reactive } from "vue";
function getLevelOption() {
    return [
        {
            itemStyle: {
                borderWidth: 0,
                gapWidth: 5
            }
        },
        {
            itemStyle: {
                gapWidth: 1
            }
        },
        {
            colorSaturation: [0.35, 0.5],
            itemStyle: {
                gapWidth: 1,
                borderColorSaturation: 0.6
            }
        }
    ];
}

const chartOptions = reactive({
    title: {
        text: 'Disk Usage',
        left: 'center'
    },
    tooltip: {
        formatter: function (info) {
            var value = info.value;
            var treePathInfo = info.treePathInfo;
            var treePath = [];
            for (var i = 1; i < treePathInfo.length; i++) {
                treePath.push(treePathInfo[i].name);
            }
            return [
                '<div class="tooltip-title">' +
                '</div>',
                'Disk Usage: ' + value + ' KB'
            ].join('');
        }
    },
    series: [
        {
            name: 'Disk Usage',
            type: 'treemap',
            visibleMin: 300,
            label: {
                show: true,
                formatter: '{b}'
            },
            itemStyle: {
                borderColor: '#fff'
            },
            levels: getLevelOption(),
            data: diskData
        }
    ]
})
</script>

<style scoped>
.chart {
    height: 800px;
    width: 800px;
}

</style>