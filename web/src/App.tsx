import '@mantine/core/styles.css';
import { MantineProvider } from '@mantine/core';
import { theme } from './theme';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { HomePage, RunScenarioPage, ViewReplay } from './pages/Home.page';
import { PlaybackManager } from './components/PlaybackManager';
import { Graph } from './pages/Graph.page';
import { ExHeader } from './components/HeaderFloating';

export default function App() {
  return (
    <MantineProvider theme={theme}>
      <RouterProvider router={router} />
    </MantineProvider>
  );
}

const router = createBrowserRouter([
  {
    path: '/:scenario/:tick?',
    element: (
      <PlaybackManager>
        <Graph />
      </PlaybackManager>
    ),
  },
  {
    path: '/',
    element: <HomePage />,
  },
  {
    path: '/run-scenario',
    element: <RunScenarioPage />,
  },
  { path: '/view-replay', element: <ViewReplay /> },
  { path: '/header', element: <ExHeader /> },
]);
