import { createAction } from 'typesafe-actions';
import {
  LolChampSelectChampSelectSession,
  LolChampSelectChampSelectSummoner,
  LolGameflowGameflowSession,
} from 'league-client';

export const championSelectSession = createAction(
  'match/championSelectsession'
)<LolChampSelectChampSelectSession>();

export const championSelectSummoner = createAction(
  'match/championSelectSummoner'
)<number, LolChampSelectChampSelectSummoner>();

export const gameflowSession = createAction('match/gameflowSession')<
  LolGameflowGameflowSession
>();
