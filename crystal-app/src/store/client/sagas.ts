import { spawn, put, call } from 'redux-saga/effects';
import { take } from 'typed-redux-saga';
import { promisified } from 'tauri/api/tauri';
import { fetchLocale } from './actions';
import { RegionLocale } from './types';

export default function* clientSaga() {
  yield spawn(fetchLocaleFlow);
}

function* fetchLocaleFlow() {
  while (true) {
    yield* take(fetchLocale.request);
    yield call(handleFetchLocale);
  }
}

function* handleFetchLocale() {
  try {
    const result: RegionLocale = yield call(promisified, {
      cmd: 'regionLocale',
    });

    yield put(fetchLocale.success(result));
  } catch (e) {
    yield put(fetchLocale.failure(e));
  }
}
