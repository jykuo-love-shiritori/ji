import { createResource } from 'solid-js';
import { z } from 'zod';
import BaseUrl from '../BaseUrl';

const Hello = z.object({
    message: z.string()
});

type Hello = z.infer<typeof Hello>;

export default () => {
    const fetchHello = async () =>
        await fetch(`${BaseUrl}/api/hello`)
            .then((res) => res.json())
            .then((json) => Hello.parse(json));

    const [data] = createResource(fetchHello);

    return (
        <>
            {data.loading && <span>Loading</span>}
            {data() && (
                <h1 class="text-3xl text-lime-500">{data()?.message}</h1>
            )}
        </>
    );
};
