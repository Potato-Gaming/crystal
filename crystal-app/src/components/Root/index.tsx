import React, { FC } from 'react';
import { Provider } from 'react-redux';
import createStore from '../../store/createStore';
import LeagueEvents from '../../leagueEvents';

const store = createStore();
const league = new LeagueEvents(store);
league.startListeners();

const Root: FC = ({ children }) => (
  <Provider store={store}>{children}</Provider>
);

export default Root;
