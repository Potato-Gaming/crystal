import React, { useEffect } from 'react';
import { listen } from 'tauri/api/event';
import { LeagueEvent } from './utils/types';
import logo from './logo.svg';
import './App.css';

function App() {
  useEffect(() => {
    const event: LeagueEvent = 'ChampionSelectBySlotId';
    listen<object>(event, (payload) => {
      console.log('PAYLOAD', payload);
    });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
