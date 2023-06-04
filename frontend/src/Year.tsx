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
import { Line } from 'solid-chartjs';

const Year = z.object({
    year: z.array(z.number().int()).nonempty(),
    total: z.array(z.number().int()).nonempty()
});

type Year = z.infer<typeof Year>;

export default () => {
    const fetchYear = async () =>
        await fetch('http://localhost:8000/api/dead_total_by_year')
            .then((res) => res.json())
            .then((json) => Year.parse(json));

    const chartData = async () => {
        const yearData = await fetchYear();

        return {
            labels: yearData.year,
            datasets: [
                {
                    label: 'Death',
                    data: yearData.total,
                    fill: true,
                    borderWidth: 1,
                    backgroundColor: (context: ScriptableContext<'line'>) => {
                        const ctx = context.chart.ctx;
                        const gradient = ctx.createLinearGradient(0, 0, 0, 200);
                        gradient.addColorStop(0, 'rgba(249, 115, 22, 1)');
                        gradient.addColorStop(1, 'rgba(249, 115, 22, 0)');
                        return gradient;
                    }
                }
            ]
        } as ChartData;
    };

    onMount(() => {
        Chart.register(Title, Tooltip, Legend, Colors, Filler);
    });

    const [data] = createResource(chartData);

    const options: ChartOptions = {
        scales: {
            y: {
                beginAtZero: true
            }
        },
        responsive: true,
        maintainAspectRatio: true,
        aspectRatio: 8 / 3
    };

    return (
        <div class="h-1/2 w-2/3">
            {data.loading && <p>Loading</p>}
            {data() && <Line data={data()} options={options} />}
        </div>
    );
};
