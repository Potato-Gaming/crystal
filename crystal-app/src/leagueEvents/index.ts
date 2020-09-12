import { listen } from 'tauri/api/event';
import {
  LeagueEventName,
  ChampionSelectBySlotId,
  ChampionSelectSesion,
  Gameflow,
  CurrentSummoner,
} from './types';
import { Store } from '../store/createStore';
import {
  championSelectSession,
  championSelectSummoner,
  gameflowSession,
} from '../store/match';

class LeagueEvents {
  store: Store;

  constructor(store: Store) {
    this.store = store;
  }

  startListeners() {
    const { dispatch } = this.store;

    const championSelectBySlotId: LeagueEventName = 'ChampionSelectBySlotId';
    listen<ChampionSelectBySlotId>(championSelectBySlotId, ({ payload }) => {
      const { ChampionSelectBySlotId: data } = payload;
      dispatch(championSelectSummoner(data[0], data[1]));
    });

    const championSelectSesion: LeagueEventName = 'ChampionSelectSesion';
    listen<ChampionSelectSesion>(championSelectSesion, ({ payload }) => {
      const { ChampionSelectSesion: data } = payload;
      dispatch(championSelectSession(data));
    });

    const gameFlow: LeagueEventName = 'Gameflow';
    listen<Gameflow>(gameFlow, ({ payload }) => {
      const { Gameflow: data } = payload;
      dispatch(gameflowSession(data));
    });

    const currentSummoner: LeagueEventName = 'CurrentSummoner';
    listen<CurrentSummoner>(currentSummoner, ({ payload }) => {
      const { CurrentSummoner: data } = payload;
      console.log('Current summoner', data);
    });
  }
}

export default LeagueEvents;
