<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import Chart from "chart.js/auto";

    let n: number = $state(1000);
    let pi: number | undefined = $state(undefined);
    let loading: boolean = $state(false);
    let error: string | undefined = $state(undefined);
    let chart: Chart | undefined = $state(undefined);
    let canvas: HTMLCanvasElement | undefined = $state(undefined);

    onMount(() => {
        if (n <= 10000) {
            initializeChart();
        }
    });

    onDestroy(() => {
        chart?.destroy();
    });

    function initializeChart() {
        if (canvas) {
            const ctx = canvas.getContext("2d");
            if (ctx) {
                chart = new Chart(ctx, {
                    type: "scatter",
                    data: {
                        datasets: [],
                    },
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                        scales: {
                            x: {
                                min: 0,
                                max: 1,
                            },
                            y: {
                                min: 0,
                                max: 1,
                            },
                        },
                        plugins: {
                            legend: {
                                display: false,
                            },
                            tooltip: {
                                enabled: false,
                            },
                        },
                    },
                });
            }
        }
    }

    async function calculatePi() {
        loading = true;
        error = undefined;
        pi = undefined;

        if (chart && n > 10000) {
            chart.destroy();
            chart = undefined;
        } else if (!chart && n <= 10000) {
            setTimeout(initializeChart, 0);
        }

        try {
            const response = await fetch(
                `http://localhost:3000/api/points?n=${n}`,
            );
            if (!response.ok) {
                throw new Error("Failed to fetch points from the backend.");
            }
            const points: { x: number; y: number }[] = await response.json();

            let inside = 0;
            const insidePoints: { x: number; y: number }[] =
                n <= 10000 ? [] : [];
            const outsidePoints: { x: number; y: number }[] =
                n <= 10000 ? [] : [];

            for (const point of points) {
                const isInside = point.x * point.x + point.y * point.y <= 1;
                if (isInside) {
                    inside++;
                }
                if (n <= 10000) {
                    if (isInside) {
                        insidePoints.push(point);
                    } else {
                        outsidePoints.push(point);
                    }
                }
            }
            pi = (4 * inside) / n;

            if (chart && n <= 10000) {
                chart.data.datasets = [
                    {
                        label: "Inside",
                        data: insidePoints,
                        backgroundColor: "rgba(75, 192, 192, 0.5)",
                    },
                    {
                        label: "Outside",
                        data: outsidePoints,
                        backgroundColor: "rgba(255, 99, 132, 0.5)",
                    },
                ];
                chart.update();
            }
        } catch (e: any) {
            error = e.message;
        } finally {
            loading = false;
        }
    }
</script>

<div class="container mx-auto p-4 max-w-4xl">
    <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
            <h1 class="card-title text-2xl">Pi Calculator</h1>
            <p>Estimate Pi using the Monte Carlo method.</p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="form-control">
                    <label class="label" for="points-input">
                        <span class="label-text">Number of points</span>
                    </label>
                    <input
                        id="points-input"
                        type="number"
                        bind:value={n}
                        class="input input-bordered"
                        min="1"
                        step="1"
                        disabled={loading}
                    />
                    <div class="card-actions justify-end mt-4">
                        <button
                            class="btn btn-primary"
                            onclick={calculatePi}
                            disabled={loading}
                        >
                            {#if loading}
                                <span class="loading loading-spinner"></span>
                                Calculating...
                            {:else}
                                Calculate
                            {/if}
                        </button>
                    </div>

                    {#if pi !== null}
                        <div class="alert alert-success mt-4">
                            <div>
                                <span>Calculation finished! Estimated Pi:</span>
                                <strong class="font-bold"
                                    >{pi?.toFixed(6)}</strong
                                >
                            </div>
                        </div>
                    {/if}

                    {#if error}
                        <div class="alert alert-error mt-4">
                            <div>
                                <span>{error}</span>
                            </div>
                        </div>
                    {/if}
                </div>
                <div
                    class="relative h-64 md:h-auto flex items-center justify-center"
                >
                    {#if n <= 10000}
                        <canvas bind:this={canvas}></canvas>
                    {:else}
                        <div class="text-center">
                            <p class="text-lg font-semibold">
                                Chart not available
                            </p>
                            <p class="text-sm">
                                To ensure performance, the chart is disabled for
                                more than 10,000 points.
                            </p>
                        </div>
                    {/if}
                </div>
            </div>
        </div>
    </div>
</div>

