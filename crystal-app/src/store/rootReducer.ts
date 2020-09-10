import { combineReducers } from 'redux';
import { History } from 'history';
import { connectRouter } from 'connected-react-router';

import matchReducer, { MatchAction } from './match/reducer';

const rootReducer = (history: History) =>
  combineReducers({
    router: connectRouter(history),
    ...matchReducer,
  });

export default rootReducer;

export type ApplicationState = ReturnType<ReturnType<typeof rootReducer>>;
export type ApplicationAction = MatchAction;
