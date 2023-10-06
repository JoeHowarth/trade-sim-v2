import {
  Paper,
  Group,
  Text,
  ActionIconGroup,
  ActionIcon,
  rem,
  Popover,
  TextInput,
} from '@mantine/core';
import { useTick, useTogglePause, useScalePlaybackSpeed, useSetTick } from './PlaybackManager';
import {
  IconPlayerPause,
  IconPlayerPlay,
  IconPlayerTrackNext,
  IconPlayerTrackPrev,
} from '@tabler/icons-react';

export function PlaybackControls() {
  const [isPaused, togglePause] = useTogglePause();
  const scale = useScalePlaybackSpeed();

  return (
    <Group>
      <TickWithPopover />
      <ActionIconGroup>
        <ActionIcon variant="default" size="lg" aria-label="Gallery">
          <IconPlayerTrackPrev
            onClick={() => scale(3 / 2)}
            style={{ width: rem(20) }}
            stroke={1.5}
          />
        </ActionIcon>

        <ActionIcon variant="default" size="lg" aria-label="Settings">
          {isPaused ? (
            <IconPlayerPlay onClick={togglePause} style={{ width: rem(20) }} stroke={1.5} />
          ) : (
            <IconPlayerPause onClick={togglePause} style={{ width: rem(20) }} stroke={1.5} />
          )}
        </ActionIcon>

        <ActionIcon variant="default" size="lg" aria-label="Likes">
          <IconPlayerTrackNext
            onClick={() => scale(2 / 3)}
            style={{ width: rem(20) }}
            stroke={1.5}
          />
        </ActionIcon>
      </ActionIconGroup>
    </Group>
  );
}

function TickWithPopover() {
  const tick = useTick();
  const tickStr = tick.toString();
  const setTick = useSetTick();

  return (
    <Popover width={100} trapFocus position="bottom" withArrow shadow="md">
      <Popover.Target>
        <div style={{ width: 80 }}>
          <Text>Tick: {tick}</Text>
        </div>
      </Popover.Target>
      <Popover.Dropdown>
        <TextInput
          label="Tick"
          placeholder={tickStr}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              const tick = parseInt(e.currentTarget.value);
              if (!Number.isNaN(tick)) setTick(tick);
            }
          }}
          size="xs"
        />
      </Popover.Dropdown>
    </Popover>
  );
}
