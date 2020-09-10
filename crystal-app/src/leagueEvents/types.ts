import {
  LolChampSelectChampSelectSession,
  LolChampSelectChampSelectSummoner,
  LolGameflowGameflowSession,
} from 'league-client';

export type LeagueEvent =
  | Gameflow
  | ChampionSelectBySlotId
  | ChampionSelectSesion;

export type Gameflow = {
  Gameflow: LolGameflowGameflowSession;
};

export type ChampionSelectBySlotId = {
  ChampionSelectBySlotId: [number, LolChampSelectChampSelectSummoner];
};

export type ChampionSelectSesion = {
  ChampionSelectSesion: LolChampSelectChampSelectSession;
};

export type LeagueEventName =
  | 'Gameflow'
  | 'ChampionSelectBySlotId'
  | 'ChampionSelectSesion';
