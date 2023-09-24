import { Welcome } from '../components/Welcome/Welcome';
import { ColorSchemeToggle } from '../components/ColorSchemeToggle/ColorSchemeToggle';
import { useEffect, useRef } from 'react';
import { Stack } from '@mantine/core';
import { network } from '@/graphics/network';

export function Graph() {
  const ref = useRef(null);
  console.log(ref);
  useEffect(() => {
    network(ref.current!);
  }, []);

  return (
    <div style={{ width: window.innerWidth, height: window.innerHeight }}>
      <canvas ref={ref} style={{ border: '1px solid black', width: '100%', height: '100%' }} />
    </div>
  );
}
