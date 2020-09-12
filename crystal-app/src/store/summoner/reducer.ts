import { combineReducers, Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import * as actions from './actions';
import { SummonerState } from './types';

export type SummonerAction = ActionType<typeof actions>;

const initialSummonerState: SummonerState = {
  status: 'idle',
  data: null,
};

export const info: Reducer<SummonerState, SummonerAction> = (
  state = initialSummonerState,
  action
) => {
  switch (action.type) {
    case getType(actions.fetchSummoner.request):
      return {
        status: 'loading',
        data: null,
      };
    case getType(actions.fetchSummoner.success):
      return {
        status: 'done',
        data: action.payload,
      };
    case getType(actions.fetchSummoner.failure):
      return {
        status: 'error',
        error: action.payload,
      };
    case getType(actions.fetchSummoner.cancel):
      return {
        status: 'abort',
        data: null,
      };
    default:
      return state;
  }
};

const summoner = combineReducers({ info });

export default { summoner };
