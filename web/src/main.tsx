import ReactDOM from 'react-dom/client';
import App from './App';

// if (import.meta.hot) {
//   //@ts-ignore
//   import.meta.hot.accept(() => import.meta.hot.invalidate());
// }

// // @ts-ignore
// if (window['reload']) window.location = window.location;
// // @ts-ignore
// window['reload'] = true;

ReactDOM.createRoot(document.getElementById('root')!).render(<App />);
