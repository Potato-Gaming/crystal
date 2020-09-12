import { combineReducers } from 'redux';
import { History } from 'history';
import { connectRouter } from 'connected-react-router';

import matchReducers, { MatchAction } from './match/reducer';
import summonerReducers, { SummonerAction } from './summoner/reducer';

const rootReducer = (history: History) =>
  combineReducers({
    router: connectRouter(history),
    ...matchReducers,
    ...summonerReducers,
  });

export default rootReducer;

export type ApplicationState = ReturnType<ReturnType<typeof rootReducer>>;
export type ApplicationAction = MatchAction | SummonerAction;
