import { LolSummonerSummoner } from 'league-client';
import { AsyncData } from '../../utils/types';

export type Summoner = LolSummonerSummoner;

export type SummonerState = AsyncData<Summoner>;
