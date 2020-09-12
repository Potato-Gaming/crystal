import { createAsyncAction } from 'typesafe-actions';
import { Summoner } from './types';

export const fetchSummoner = createAsyncAction(
  'summoner/fetch',
  'summoner/fetch_success',
  'summoner/fetch_error',
  'summoner/fetch_abort'
)<void, Summoner, Error, void>();
