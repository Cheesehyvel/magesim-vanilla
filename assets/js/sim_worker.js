import init, { run_simulation, run_simulations } from "simulator";

onmessage = (event) => {
    if (event.data.type == "start") {
        init().then(r => {

            let result;
            if (event.data.iterations == 1)
                result = run_simulation(event.data.config);
            else
                result = run_simulations(event.data.config, event.data.iterations);

            postMessage({
                type: "success",
                result: result,
            });
        })
        .catch(e => {
            console.warn(e);
        });
    }
}