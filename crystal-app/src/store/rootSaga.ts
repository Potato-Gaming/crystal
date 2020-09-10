import { all, spawn } from 'redux-saga/effects';
import matchSaga from './match/sagas';

export default function* rootSata() {
  yield all([spawn(matchSaga)]);
}
