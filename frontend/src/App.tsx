import { Component } from 'solid-js';
import { A, Route, Routes } from '@solidjs/router';
import Hello from './Hello';
import Dead from './Dead';

const App: Component = () => {
    return (
        <div class="flex flex-col items-center">
            <div class="my-10 flex space-x-5">
                <A href="/" class="text-4xl">
                    Hello
                </A>
                <A href="/dead" class="text-4xl">
                    Dead
                </A>
            </div>

            <Routes>
                <Route path="/" component={Hello} />
                <Route path="/dead" component={Dead} />
            </Routes>
        </div>
    );
};

export default App;
