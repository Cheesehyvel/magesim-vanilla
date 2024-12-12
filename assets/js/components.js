import AnimateNumber from "./Components/AnimateNumber.vue";
import Checkbox from "./Components/Checkbox.vue";
import CombatChart from "./Components/CombatChart.vue";
import Help from "./Components/Help.vue";
import Micon from "./Components/Micon.vue";
import ProgressCircle from "./Components/ProgressCircle.vue";
import SelectSimple from "./Components/SelectSimple.vue";
import SortLink from "./Components/SortLink.vue";
import SpellPower from "./Components/SpellPower.vue";
import Spotlight from "./Components/Spotlight.vue";
import TalentCalculator from "./Components/TalentCalculator.vue";
import Tooltip from "./Components/Tooltip.vue";
import Wowicon from "./Components/Wowicon.vue";

export default {
    install(app) {
        app.component("animate-number", AnimateNumber);
        app.component("checkbox", Checkbox);
        app.component("combat-chart", CombatChart);
        app.component("help", Help);
        app.component("micon", Micon);
        app.component("progress-circle", ProgressCircle);
        app.component("select-simple", SelectSimple);
        app.component("sort-link", SortLink);
        app.component("spell-power", SpellPower);
        app.component("spotlight", Spotlight);
        app.component("talent-calculator", TalentCalculator);
        app.component("tooltip", Tooltip);
        app.component("wowicon", Wowicon);
    }
};