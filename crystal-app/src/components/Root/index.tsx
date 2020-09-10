import React, { FC } from 'react';
import { Provider } from 'react-redux';
import { ConnectedRouter } from 'connected-react-router';
import { createBrowserHistory } from 'history';
import createStore from '../../store/createStore';
import LeagueEvents from '../../leagueEvents';

const history = createBrowserHistory();
const store = createStore(history);
const league = new LeagueEvents(store);
league.startListeners();

const Root: FC = ({ children }) => (
  <Provider store={store}>
    <ConnectedRouter history={history}>{children}</ConnectedRouter>
  </Provider>
);

export default Root;
