import { Component } from 'solid-js';
import { A, Route, Routes } from '@solidjs/router';
import Hello from './Hello';
import Dead from './Dead';

const App: Component = () => {
    return (
        <div class="h-screen bg-neutral-900 font-bold text-neutral-200">
            <div class="flex flex-col items-center">
                <div class="my-10 flex space-x-5">
                    <A
                        href="/"
                        class="text-4xl transition hover:text-neutral-400"
                    >
                        Hello
                    </A>
                    <A
                        href="/dead"
                        class="text-4xl transition hover:text-neutral-400"
                    >
                        Dead
                    </A>
                </div>

                <Routes>
                    <Route path="/" component={Hello} />
                    <Route path="/dead" component={Dead} />
                </Routes>
            </div>
        </div>
    );
};

export default App;
