import { createAction } from 'typesafe-actions';
import {
  ChampSelectSession,
  GameflowSession,
  ChampSelectSummoner,
} from './types';

export const championSelectSession = createAction(
  'match/championSelectsession'
)<ChampSelectSession>();

export const championSelectSummoner = createAction(
  'match/championSelectSummoner'
)<number, ChampSelectSummoner>();

export const gameflowSession = createAction('match/gameflowSession')<
  GameflowSession
>();
