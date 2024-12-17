import Worker from "./sim_worker.js?worker";

class SimContainer {
    constructor(threads, iterations, config, onSuccess, onError, onProgress) {
        this.config = _.cloneDeep(config);
        this.threads = threads;
        this.iterations = parseInt(iterations);
        this.workers = [];
        this.runs = [];
        this.start_time = null;

        if (!this.threads || isNaN(this.threads))
            throw "Invalid threads";
        if (!this.iterations || isNaN(this.iterations))
            throw "Invalid iterations";

        let run_itr = Math.max(50, Math.min(200, Math.ceil(this.iterations/this.threads)));
        let num_workers = Math.min(this.threads, Math.ceil(this.iterations/run_itr));

        for (let total = 0; total < this.iterations; total+= run_itr) {
            run_itr = Math.min(run_itr, this.iterations - total);
            this.runs.push({
                iterations: run_itr,
                started: false,
            });
        }

        let sum = null;

        for (let i=0; i<num_workers; i++) {
            this.workers.push(new Worker());
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
                        sum.dps = (sum.dps * sum.iterations + data.result.dps * data.result.iterations) / (sum.iterations + data.result.iterations);
                        sum.ignite_dps = (sum.ignite_dps * sum.iterations + data.result.ignite_dps * data.result.iterations) / (sum.iterations + data.result.iterations);

                        for (let j=0; j<sum.players.length; j++) {
                            sum.players[j].dps = (sum.players[j].dps * sum.iterations + data.result.players[j].dps * data.result.iterations) / (sum.iterations + data.result.iterations);
                        }

                        sum.iterations+= data.result.iterations;
                    }

                    if (onProgress)
                        onProgress(sum);

                    if (this.iterations == 1 || sum.iterations == this.iterations) {
                        this.workers[i].terminate();
                        sum.time = (Date.now() - this.start_time) / 1000;
                        onSuccess(sum);
                    }
                    else {
                        if (!this.startNextRun(i))
                            this.workers[i].terminate();
                    }
                }
            };

            this.workers[i].onerror = (...args) => {
                onError(...args);
                this.workers[i].terminate();
            };
        }
    }

    start() {
        this.start_time = Date.now();
        for (let i=0; i<this.workers.length; i++)
            this.startRun(i, i);
    }

    startRun(worker_index, run_index) {
        let run = this.runs[run_index];
        let config = _.cloneDeep(this.config);
        config.rng_seed = config.rng_seed + run_index * run.iterations;

        this.workers[worker_index].postMessage({
            type: "start",
            config: config,
            iterations: run.iterations,
        });
        run.started = true;
    }

    startNextRun(worker_index) {
        for (let i=0; i<this.runs.length; i++) {
            if (!this.runs[i].started) {
                this.startRun(worker_index, i);
                return true;
            }
        }

        return false;
    }
}

export default SimContainer;