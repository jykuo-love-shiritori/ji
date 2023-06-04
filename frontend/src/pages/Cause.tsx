import { createResource, onMount } from 'solid-js';
import { z } from 'zod';
import {
    Chart,
    Title,
    Tooltip,
    Legend,
    Colors,
    Filler,
    ChartData,
    ChartOptions,
    ScriptableContext
} from 'chart.js';
import { Pie } from 'solid-chartjs';

const Cause = z.object({
    cause: z.array(z.string()),
    total: z.array(z.number().int())
});

type Cause = z.infer<typeof Cause>;

export default () => {
    const fetchChartData = async () => {
        const raw = await fetch('http://localhost:8000/api/dead_total_by_cause')
            .then((res) => res.json())
            .then((json) => Cause.parse(json));

        return {
            labels: raw.cause,
            datasets: [
                {
                    label: 'Death',
                    data: raw.total,
                    fill: true,
                    borderWidth: 1,
                    hoverOffset: 20,
                    borderColor: [
                        '#84cc16',
                        '#4ade80',
                        '#2dd4bf',
                        '#38bdf8',
                        '#8b5cfd',
                        '#d946ef',
                        '#ec4899',
                        '#d4d4d4'
                    ],
                    backgroundColor: [
                        '#1a2e05',
                        '#052e16',
                        '#042f2e',
                        '#082f49',
                        '#1e1b4b',
                        '#4a044e',
                        '#500724',
                        '#171717'
                    ]
                }
            ]
        } as ChartData;
    };

    onMount(() => {
        Chart.register(Title, Tooltip, Legend, Colors, Filler);
    });

    const [data] = createResource(fetchChartData);

    const options: ChartOptions = {
        responsive: true,
        maintainAspectRatio: true,
        aspectRatio: 3 / 2
    };

    return (
        <div class="w-1/2">
            {data.loading && <p>Loading</p>}
            {data() && <Pie data={data()} options={options} />}
        </div>
    );
};
