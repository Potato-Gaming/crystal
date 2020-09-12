import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { Switch, Route } from 'react-router';
import Home from './views/Home';
import ChampionSelect from './views/ChampionSelect';
import { fetchSummoner } from './store/summoner';
import './app.css';

function App({ fetchSummoner }: Props) {
  console.log('render app');
  useEffect(() => {
    console.log('fetchSummoner', fetchSummoner);
    fetchSummoner();
  }, [fetchSummoner]);

  return (
    <div className="app" data-testid="app">
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

const mapDispatchToProps = {
  fetchSummoner: fetchSummoner.request,
};

type Props = typeof mapDispatchToProps;

export default connect(null, mapDispatchToProps)(App);
