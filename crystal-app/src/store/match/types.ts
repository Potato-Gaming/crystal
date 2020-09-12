import {
  LolGameflowGameflowPhase,
  LolChampSelectChampSelectSession,
  LolGameflowGameflowSession,
  LolChampSelectChampSelectSummoner,
} from 'league-client';

export type ChampSelectSession = LolChampSelectChampSelectSession;
export type GameflowSession = LolGameflowGameflowSession;
export type GamePhase = LolGameflowGameflowPhase;
export type ChampSelectSummoner = LolChampSelectChampSelectSummoner;

export type MatchPhaseState = GamePhase | null;
