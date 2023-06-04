import { A } from '@solidjs/router';

export default () => {
    return (
        <div class="my-10 flex space-x-5">
            <A
                href="/"
                class="text-4xl font-black transition hover:text-neutral-400"
            >
                Hello
            </A>
            <A
                href="/year"
                class="text-4xl font-black transition hover:text-neutral-400"
            >
                Year
            </A>
            <A
                href="/cause"
                class="text-4xl font-black transition hover:text-neutral-400"
            >
                Cause
            </A>
            <A
                href="/sex"
                class="text-4xl font-black transition hover:text-neutral-400"
            >
                Sex
            </A>
        </div>
    );
};
