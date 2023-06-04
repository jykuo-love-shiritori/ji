import { Component } from 'solid-js';
import { Route, Routes } from '@solidjs/router';
import Header from './components/Header';
import Hello from './pages/Hello';
import Year from './pages/Year';
import Cause from './pages/Cause';
import Sex from './pages/Sex';

const App: Component = () => {
    return (
        <div class="bg-neutral-950 h-screen font-bold text-white">
            <div class="flex flex-col items-center">
                <Header />

                <Routes>
                    <Route path="/" component={Hello} />
                    <Route path="/year" component={Year} />
                    <Route path="/cause" component={Cause} />
                    <Route path="/sex" component={Sex} />
                </Routes>
            </div>
        </div>
    );
};

export default App;
