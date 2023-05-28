import { Component, createResource } from 'solid-js';
import { z } from 'zod';

const Hello = z.object({
    message: z.string()
});

type Hello = z.infer<typeof Hello>;

const App: Component = () => {
    const fetchHello = async () =>
        await fetch('http://localhost:8000/hello')
            .then((res) => res.json())
            .then((json) => Hello.parse(json));

    const [data] = createResource(fetchHello);

    return (
        <div class="flex flex-col items-center">
            <p class="py-20 text-center text-4xl text-green-700">
                Hello tailwind!
            </p>

            {data.loading && <span>Loading</span>}
            {data() && <h1 class="text-3xl">{data()?.message}</h1>}
        </div>
    );
};

export default App;
