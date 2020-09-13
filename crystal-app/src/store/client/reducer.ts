import { combineReducers, Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import * as actions from './actions';
import { RegionLocaleState } from './types';

export type ClientAction = ActionType<typeof actions>;

const initialSummonerState: RegionLocaleState = {
  status: 'idle',
  data: null,
};

export const locale: Reducer<RegionLocaleState, ClientAction> = (
  state = initialSummonerState,
  action
) => {
  switch (action.type) {
    case getType(actions.fetchLocale.request):
      return {
        status: 'loading',
        data: null,
      };
    case getType(actions.fetchLocale.success):
      return {
        status: 'done',
        data: action.payload,
      };
    case getType(actions.fetchLocale.failure):
      return {
        status: 'error',
        error: action.payload,
      };
    case getType(actions.fetchLocale.cancel):
      return {
        status: 'abort',
        data: null,
      };
    default:
      return state;
  }
};

const client = combineReducers({ locale });

export default { client };
