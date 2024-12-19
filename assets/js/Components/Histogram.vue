<script setup>
import {
  Chart,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
} from "chart.js";
import { Bar } from "vue-chartjs";
import { ref, onMounted, watch, toRaw } from "vue";

Chart.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

const props = defineProps(["data"]);

const chartData = ref(null);
const chartOptions = ref(null);
const makeChart = () => {
    let map = toRaw(props.data);
    let keys = map.keys().toArray().sort();
    let binMax = _.last(keys);
    let binSize = 50000;
    for (let i=1; i<keys.length; i++) {
        let diff = keys[i] - keys[i-1];
        if (diff < binSize)
            binSize = diff;
    }

    let data = {
        labels: [],
        datasets: [{
            data: [],
            label: "Iterations",
            backgroundColor: "#08f",
            borderColor: "#05c",
            borderWidth: 1,
            barPercentage: 1.25,
        }],
    };
    let options = {
        legend: {
            display: false,
        },
        maintainAspectRatio: false,
        scales: {
            x: {
                display: true,
                type: "linear",
                max: binMax,
                title: {
                    display: true,
                    text: "DPS",
                }
            },
            y: {
                type: "linear",
                beginAtZero: true,
                title: {
                    display: true,
                    text: "Iterations",
                },
                grid: {
                    color: "rgba(120,140,240,0.2)",
                },
            },
        },
        plugins: {
            tooltip: {
                callbacks: {
                    title: (context) => {
                        return context[0].label+" dps";
                    },
                    label: (context) => {
                        return context.parsed.y+" iterations";
                    }
                }
            },
            legend: {
                display: false,
            }
        }
    };

    if (keys.length) {
        for (let bin = keys[0]; bin<binMax; bin+= binSize) {
            let value = map.get(bin);
            data.labels.push(bin);
            data.datasets[0].data.push(value ? value : 0);
        }
    }

    chartData.value = data;
    chartOptions.value = options;
};

onMounted(() => {
    makeChart();
});

watch(() => props.data, makeChart);
</script>

<template>
    <div class="histogram-chart">
        <div class="chart" v-if="chartData">
            <Bar :data="chartData" :options="chartOptions" />
        </div>
    </div>
</template>