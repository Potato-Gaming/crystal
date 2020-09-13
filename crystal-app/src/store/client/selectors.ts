import { ApplicationState } from '../index';

export const selectLocale = (state: ApplicationState) => state.client.locale;
