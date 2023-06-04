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
    year: z.array(z.number().int()),
    total: z.array(z.number().int())
});

type Year = z.infer<typeof Year>;

export default () => {
    const fetchChartData = async () => {
        const raw = await fetch('http://localhost:8000/api/dead_total_by_year')
            .then((res) => res.json())
            .then((json) => Year.parse(json));

        return {
            labels: raw.year,
            datasets: [
                {
                    label: 'Death',
                    data: raw.total,
                    fill: true,
                    borderWidth: 1,
                    borderColor: '#84cc16',
                    tension: 0.3,
                    backgroundColor: (context: ScriptableContext<'line'>) => {
                        const ctx = context.chart.ctx;
                        const gradient = ctx.createLinearGradient(0, 0, 0, 300);
                        gradient.addColorStop(0, '#36531499');
                        gradient.addColorStop(1, '#36531400');
                        return gradient;
                    }
                }
            ]
        } as ChartData;
    };

    onMount(() => {
        Chart.register(Title, Tooltip, Legend, Colors, Filler);
    });

    const [data] = createResource(fetchChartData);

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
