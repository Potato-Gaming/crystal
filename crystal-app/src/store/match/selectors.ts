import { ApplicationState } from '../index';

export const selectMatchPhase = (state: ApplicationState) => state.match.phase;
