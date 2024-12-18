import AnimateNumber from "./Components/AnimateNumber.vue";
import Apl from "./Components/Apl.vue";
import AplAction from "./Components/AplAction.vue";
import AplCondition from "./Components/AplCondition.vue";
import AplValue from "./Components/AplValue.vue";
import Checkbox from "./Components/Checkbox.vue";
import CombatChart from "./Components/CombatChart.vue";
import Help from "./Components/Help.vue";
import Histogram from "./Components/Histogram.vue";
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
        app.component("apl", Apl);
        app.component("apl-action", AplAction);
        app.component("apl-condition", AplCondition);
        app.component("apl-value", AplValue);
        app.component("checkbox", Checkbox);
        app.component("combat-chart", CombatChart);
        app.component("help", Help);
        app.component("histogram", Histogram);
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