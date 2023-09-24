import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { HomePage } from './pages/Home.page';
import { Graph } from './pages/Graph.page';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Graph />,
  },
  {
    path: '/homepage',
    element: <HomePage />,
  },
]);

export function Router() {
  return <RouterProvider router={router} />;
}
