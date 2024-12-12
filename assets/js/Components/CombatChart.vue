<script setup>
import {
  Chart,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Legend
} from "chart.js";
import { Line } from "vue-chartjs";
import { ref, computed, onMounted, watch } from "vue";

Chart.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Legend
);

const props = defineProps(["result", "player"]);

const chartData = ref(null);
const chartOptions = ref(null);
const makeChart = () => {
    let data = {
        labels: [],
        datasets: [],
    };
    let options = {
        maintainAspectRatio: false,
        elements: {
            point: {
                radius: 0,
            },
        },
        scales: {
            x: {
                display: true,
                type: "linear",
                max: props.result.t,
                title: {
                    display: true,
                    text: "Time (s)",
                },
                grid: {
                    color: "rgba(120,140,240,0.1)",
                }
            },
            dps: {
                type: "linear",
                beginAtZero: true,
                title: {
                    display: true,
                    text: "DPS",
                },
                grid: {
                    color: "rgba(120,140,240,0.2)",
                },
            },
            mana: {
                type: "linear",
                beginAtZero: true,
                max: 100,
                display: false,
                title: {
                    display: true,
                    text: "Mana %",
                },
            },
        },
        // plugins: {
        //     tooltip: {
        //         callbacks: {
        //             title: (context) => {
        //                 return `Time: ${context[0].label} seconds`;
        //             },
        //             label: (context) => {
        //                 return `${context.dataset.label}: ${context.parsed.y.toFixed(1)}`;
        //             }
        //         }
        //     }
        // }
    };

    let mana_smooth = false;
    let d = [];
    let names = _.map(_.uniqBy(props.result.log, "unit_name"), "unit_name");
    let log = props.result.log;
    let sublog = [];
    if (props.player) {
        names = names.filter(n => n == props.player);
        log = log.filter(l => l.unit_name == props.player);
    }

    // Total dps
    // d = [];
    // d.push({x: 0, y: 0});
    // for (let entry of log) {
    //     if (entry.log_type == "SpellImpact")
    //         d.push({x: entry.t, y: entry.total_dps});
    // }
    // d.push({x: props.result.t, y: props.result.dps});
    // data.datasets.push({
    //     data: d,
    //     borderColor: "#d00",
    //     backgroundColor: "#d00",
    //     yAxisID: "dps",
    //     label: "Total dps",
    // });

    // DPS
    let colors = [
        "#d00",
        "#dd0",
        "#80a",
        "#0a0",
        "#fa0",
        "#0cc",
        "#eee",
    ];
    let index = 0;
    for (let name of names) {
        sublog = props.player ? log : log.filter(l => l.unit_name == name);
        d = [];
        d.push({x: 0, y: 0});
        for (let entry of sublog) {
            if (entry.log_type == "SpellImpact")
                d.push({x: entry.t, y: entry.dps});
        }
        d.push({x: props.result.t, y: _.find(props.result.players, {name: name}).dps});
        data.datasets.push({
            data: d,
            borderColor: colors[index % colors.length],
            backgroundColor: colors[index % colors.length],
            yAxisID: "dps",
            label: props.player ? "DPS" : name,
        });
        index++;
    }

    // Mana
    // Only have mana graph for a single player
    if (props.player) {
        for (let name of names) {
            sublog = props.player ? log : log.filter(l => l.unit_name == name);
            d = [];
            d.push({x: 0, y: 100});
            if (mana_smooth) {
                for (let entry of sublog) {
                    if (entry.t < 0)
                        continue;
                    if (entry.log_type == "Mana")
                        d.push({x: entry.t, y: entry.mana_percent});
                }
                d.push({x: props.result.t, y: _.last(sublog.filter(l => l.unit_name == name)).mana_percent});
            }
            else {
                for (let entry of log) {
                    if (entry.t < 0)
                        continue;
                    d.push({x: entry.t, y: entry.mana_percent});
                }
            }
            data.datasets.push({
                data: d,
                borderColor: "#08f",
                backgroundColor: "#08f",
                yAxisID: "mana",
                label: "Mana",
            })
        }
    }
    else {
        delete options.scales.mana;
    }

    chartData.value = data;
    chartOptions.value = options;
};

onMounted(() => {
    makeChart();
});

watch(() => props.result, makeChart);
watch(() => props.player, makeChart);
</script>

<template>
    <div class="combat-chart">
        <div class="chart" v-if="chartData">
            <Line :data="chartData" :options="chartOptions" />
        </div>
    </div>
</template>