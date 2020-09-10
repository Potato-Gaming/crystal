import { combineReducers } from 'redux';

import matchReducer, { MatchAction } from './match/reducer';

const rootReducer = () =>
  combineReducers({
    ...matchReducer,
  });

export default rootReducer;

export type ApplicationState = ReturnType<ReturnType<typeof rootReducer>>;
export type ApplicationAction = MatchAction;
