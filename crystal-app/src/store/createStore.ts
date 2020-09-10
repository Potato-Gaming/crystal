import { createStore, applyMiddleware } from 'redux';
import createSagaMiddleware from 'redux-saga';
import rootReducer from './rootReducer';
import rootSaga from './rootSaga';

const sagaMiddleware = createSagaMiddleware({});

const buildStore = () => {
  const store = createStore(rootReducer(), {}, applyMiddleware(sagaMiddleware));

  sagaMiddleware.run(rootSaga);

  global.__redux_store__ = store;

  return store;
};

export type Store = ReturnType<typeof buildStore>;

export default buildStore;
