import Worker from "./sim_worker.js?worker";

class SimContainer {
    constructor(threads, iterations, onSuccess, onError) {
        this.threads = threads;
        this.workers = Array(this.threads);
        this.iterations = parseInt(iterations);
        this.start_time = null;

        if (!this.threads || isNaN(this.threads))
            throw "Invalid threads";
        if (!this.iterations || isNaN(this.iterations))
            throw "Invalid iterations";

        let sum = null;

        for (let i=0; i<this.threads; i++) {
            this.workers[i] = new Worker();
            this.workers[i].onmessage = (event) => {
                let data = event.data;

                if (data.type == "error") {
                    this.workers[i].terminate();
                    onError(data);
                }

                // Thread done
                if (data.type == "success") {
                    // Merge results
                    if (!sum) {
                        sum = data.result;
                    }
                    else {
                        if (data.result.min_dps < sum.min_dps)
                            sum.min_dps = data.result.min_dps;
                        if (data.result.max_dps > sum.max_dps)
                            sum.max_dps = data.result.max_dps;
                        sum.avg_dps = (sum.avg_dps * sum.iterations + data.result.avg_dps * data.result.iterations) / (sum.iterations + data.result.iterations);

                        sum.iterations+= data.result.iterations;
                    }

                    this.workers[i].terminate();

                    if (this.iterations == 1 || sum.iterations == this.iterations) {
                        sum.time = (Date.now() - this.start_time) / 1000;
                        onSuccess(sum);
                    }
                }
            };

            this.workers[i].onerror = (...args) => {
                onError(...args);
                this.workers[i].terminate();
            };
        }
    }

    start(config) {
        config = _.cloneDeep(config);
        let seed = config.rng_seed;
        this.start_time = Date.now();
        for (let i=0; i<this.workers.length; i++) {
            let worker_iterations = Math.floor((this.iterations+i)/this.threads);
            if (worker_iterations > 0) {
                if (config.rng_seed > 0)
                    config.rng_seed+= worker_iterations;

                this.workers[i].postMessage({
                    type: "start",
                    config: config,
                    iterations: worker_iterations,
                });
            }
        }
    }
}

export default SimContainer;