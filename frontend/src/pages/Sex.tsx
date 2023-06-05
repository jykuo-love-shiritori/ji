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
    ChartOptions
} from 'chart.js';
import { Bar } from 'solid-chartjs';
import BaseUrl from '../BaseUrl';

const Cause = z.object({
    age_code: z.array(z.string()),
    total_male: z.array(z.number().int()),
    total_female: z.array(z.number().int())
});

type Cause = z.infer<typeof Cause>;

export default () => {
    const fetchChartData = async () => {
        const raw = await fetch(`${BaseUrl}/api/dead_total_by_age_code`)
            .then((res) => res.json())
            .then((json) => Cause.parse(json));

        return {
            labels: raw.age_code,
            datasets: [
                {
                    label: 'Male',
                    data: raw.total_male,
                    borderWidth: 1,
                    borderColor: '#06b6d4',
                    backgroundColor: '#083344'
                },
                {
                    label: 'Female',
                    data: raw.total_female.map((n) => -n),
                    borderWidth: 1,
                    borderColor: '#db2777',
                    backgroundColor: '#500724'
                }
            ]
        } as ChartData;
    };

    onMount(() => {
        Chart.register(Title, Tooltip, Legend, Colors, Filler);
    });

    const [data] = createResource(fetchChartData);

    const options: ChartOptions = {
        indexAxis: 'y',
        scales: {
            y: {
                stacked: true
            },
            x: {
                stacked: true,
                ticks: {
                    callback: (v, _) =>
                        typeof v === 'number' ? Math.abs(v) : undefined
                },
                max: 400000,
                min: -400000
            }
        },
        plugins: {
            legend: {
                position: 'right'
            }
        },
        responsive: true,
        maintainAspectRatio: true,
        aspectRatio: 2 / 1
    };

    return (
        <div class="w-2/3">
            {data.loading && <p>Loading</p>}
            {data() && <Bar data={data()} options={options} />}
        </div>
    );
};
