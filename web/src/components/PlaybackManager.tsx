import { useRef, useState, useEffect, createContext, useContext } from 'react';
import { useParams, useNavigate } from 'react-router-dom';

export type PlaybackMsg = {
  tick?: number;
  ms?: number;
};

type SendType = undefined | ((msg: PlaybackMsg) => void);
type PlaybackContextType = { send: SendType; ms: number };

export const PlaybackContext = createContext<PlaybackContextType>({
  send: (msg: Record<string, any>) => {},
  ms: 0,
});

export function PlaybackManager({ children }: React.PropsWithChildren) {
  const params = useParams<'tick'>();
  const tick = params.tick ?? 0;
  const replay = useReplay();

  const ws = useRef<any>(null);
  const navigate = useNavigate();
  const [send, setSend] = useState<SendType>(undefined);
  const [ms, setMs] = useState(0);

  useEffect(() => {
    console.log('effect running');
    const socket = new WebSocket('ws://127.0.0.1:8000/ticks/' + tick);

    socket.onopen = () => {
      console.log('opened');
      setSend(() => (msg: PlaybackMsg) => socket.send(JSON.stringify(msg)));
    };

    socket.onclose = () => {
      console.log('closed');
    };

    socket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      console.log('got json', data);
      setMs((ms) => {
        return data.ms;
      });
      navigate(`/${replay}/${data.tick}`, { replace: true });
    };

    ws.current = socket;

    return () => {
      console.log('closing');
      socket.close();
    };
  }, []);

  return <PlaybackContext.Provider value={{ send, ms }}>{children}</PlaybackContext.Provider>;
}

export function useReplay() {
  return useParams<'scenario'>().scenario!;
}

export function useTick() {
  const { tick } = useParams<'tick'>();
  return parseInt(tick ?? '0');
}

export function useSetTick() {
  const navigate = useNavigate();
  const replay = useReplay();
  const { send } = useContext(PlaybackContext);
  return (tick: number) => {
    navigate(`/${replay}/${tick}`, { replace: true });
    if (send) send({ tick: tick });
  };
}

export function useTogglePause(): [boolean, () => void] {
  const { send, ms } = useContext(PlaybackContext);
  const [prev, setPrev] = useState(ms);
  return [
    ms === 0,
    () => {
      if (send) {
        console.log('ms', ms, 'prev', prev);
        if (ms === 0) {
          // unpause
          console.log('unpause', prev > 0 ? prev : 1000);
          send({ ms: prev });
        } else {
          // pause
          console.log('pause', 0);
          send({ ms: 0 });
        }
        setPrev(ms);
      }
    },
  ];
}

export function useScalePlaybackSpeed() {
  const { send, ms } = useContext(PlaybackContext);
  // const [target, setTarget] = useState(ms);
  // const [old, setOld] = useState(ms);
  return (scaleFactor: number) => {
    if (send) {
      // setOld(ms);

      // const target = old * scaleFactor;

      send({ ms: ms * scaleFactor });
    }
  };
}
