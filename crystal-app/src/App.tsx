import React from 'react';
import { Switch, Route } from 'react-router';
import Home from './views/Home';
import ChampionSelect from './views/ChampionSelect';

function App() {
  return (
    <div data-testid="app">
      <Switch>
        <Route exact path="/">
          <Home />
        </Route>
        <Route exact path="/champion-select">
          <ChampionSelect />
        </Route>
      </Switch>
    </div>
  );
}

export default App;
