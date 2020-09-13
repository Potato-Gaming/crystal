import { all, spawn } from 'redux-saga/effects';
import clientSaga from './client/sagas';
import matchSaga from './match/sagas';
import summonerSaga from './summoner/sagas';

export default function* rootSata() {
  yield all([spawn(clientSaga), spawn(matchSaga), spawn(summonerSaga)]);
}
