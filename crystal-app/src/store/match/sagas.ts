import { takeEvery } from 'redux-saga/effects';
import { select, put } from 'typed-redux-saga';
import { getType } from 'typesafe-actions';
import { ActionMatchingPattern as ActionType } from '@redux-saga/types';
import { getLocation, push } from 'connected-react-router';
import { LolGameflowGameflowPhase as Phase } from 'league-client';
import { gameflowSession } from './actions';

export default function* matchRootSaga() {
  yield takeEvery(getType(gameflowSession), handleGameflowSession);
}

function* handleGameflowSession(action: ActionType<typeof gameflowSession>) {
  const location = yield* select(getLocation);
  const { payload } = action;

  if (
    payload.phase === Phase.ChampSelect &&
    location.pathname !== '/champion-select'
  ) {
    yield put(push('/champion-select'));
  }

  if (payload.phase === Phase.None && location.pathname !== '/') {
    yield put(push('/'));
  }
}
