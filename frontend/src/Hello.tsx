import { createResource } from 'solid-js';
import { z } from 'zod';

const Hello = z.object({
    message: z.string()
});

type Hello = z.infer<typeof Hello>;

export default () => {
    const fetchHello = async () =>
        await fetch('http://localhost:8000/api/hello')
            .then((res) => res.json())
            .then((json) => Hello.parse(json));

    const [data] = createResource(fetchHello);

    return (
        <>
            {data.loading && <span>Loading</span>}
            {data() && <h1 class="text-3xl">{data()?.message}</h1>}
        </>
    );
};
