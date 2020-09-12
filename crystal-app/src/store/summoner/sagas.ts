import { spawn, put, call } from 'redux-saga/effects';
import { take } from 'typed-redux-saga';
import { promisified } from 'tauri/api/tauri';
import { fetchSummoner } from './actions';
import { Summoner } from './types';

export default function* matchSummonerSaga() {
  yield spawn(fetchSummonerFlow);
}

function* fetchSummonerFlow() {
  while (true) {
    yield* take(fetchSummoner.request);
    console.log('fetch summoner');

    yield call(handleFetchSummoner);
  }
}

function* handleFetchSummoner() {
  try {
    const result: Summoner = yield call(promisified, {
      cmd: 'currentSummoner',
    });
    console.log('RESULT', result);
  } catch (e) {
    yield put(fetchSummoner.failure(e));
  }
}
