import { Component } from 'solid-js';
import { A, Route, Routes } from '@solidjs/router';
import Hello from './Hello';
import Year from './Year';

const App: Component = () => {
    return (
        <div class="bg-neutral-950 h-screen font-bold text-white">
            <div class="flex flex-col items-center">
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
                </div>

                <Routes>
                    <Route path="/" component={Hello} />
                    <Route path="/year" component={Year} />
                </Routes>
            </div>
        </div>
    );
};

export default App;
