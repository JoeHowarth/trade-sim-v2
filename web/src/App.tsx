import '@mantine/core/styles.css';
import { MantineProvider } from '@mantine/core';
import { theme } from './theme';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { HomePage } from './pages/Home.page';
import { PlaybackManager } from './components/PlaybackManager';
import { PlaybackControls } from './components/PlaybackControls';
import { Graph } from './pages/Graph.page';

export default function App() {
  return (
    <MantineProvider theme={theme}>
      <RouterProvider router={router} />
    </MantineProvider>
  );
}

const router = createBrowserRouter([
  {
    path: '/:tick?',
    element: (
      <PlaybackManager>
        <Graph />
      </PlaybackManager>
    ),
  },
  {
    path: '/homepage',
    element: <HomePage />,
  },
]);
