import { Component } from 'solid-js';
import Hello from './Hello';

const App: Component = () => {
    return (
        <div class="flex flex-col items-center">
            <p class="py-20 text-center text-4xl text-green-700">
                Hello tailwind!
            </p>

            <Hello />
        </div>
    );
};

export default App;
