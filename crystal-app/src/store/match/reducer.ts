import { combineReducers, Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import * as actions from './actions';
import { MatchPhaseState } from './types';

export type MatchAction = ActionType<typeof actions>;

const initialPhaseState = null;

export const phase: Reducer<MatchPhaseState, MatchAction> = (
  state = initialPhaseState,
  action
) => {
  switch (action.type) {
    case getType(actions.gameflowSession):
      return action.payload.phase || null;
    default:
      return state;
  }
};

const match = combineReducers({ phase });

export default { match };
