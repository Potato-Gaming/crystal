import { createAsyncAction } from 'typesafe-actions';
import { RegionLocale } from './types';

export const fetchLocale = createAsyncAction(
  'client/fetch_locale',
  'client/fetch_locale_success',
  'client/fetch_locale_error',
  'client/fetch_locale_abort'
)<void, RegionLocale, Error, void>();
