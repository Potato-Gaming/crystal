import { all, spawn } from 'redux-saga/effects';
import matchSaga from './match/sagas';
import summonerSaga from './summoner/sagas';

export default function* rootSata() {
  yield all([spawn(matchSaga), spawn(summonerSaga)]);
}
