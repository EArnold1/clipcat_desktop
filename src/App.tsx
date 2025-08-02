import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import 'src/App.css';
import { Clips } from 'components/clip';

function App() {
  const [items, setItems] = useState<Array<{ id: string; value: string }>>([]);

  async function load_clips() {
    const clips = await invoke('load_clips');

    setItems(clips as typeof items);
  }

  useEffect(() => {
    load_clips();
  }, []);

  return (
    <main className="w-full overflow-hidden relative">
      <section className="fixed w-full bg-white">
        {/* Header */}
        <div className="flex items-center px-4 py-3 border-b border-gray-200 gap-2">
          <h4 className="flex-1 text-sm font-semibold">Clipcat</h4>

          <form
            id="search-form"
            className="flex-1 flex gap-x-3 items-center border rounded-md bg-gray-50 border-gray-300 px-2"
          >
            <input
              id="search-input"
              type="text"
              placeholder="Search"
              aria-label="Search clipboard"
              className="w-full text-xs outline-none py-2 bg-inherit"
            />
            <button className="flex items-center">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-4 w-4"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                strokeWidth="2"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                />
              </svg>
            </button>
          </form>
        </div>

        {/* Controls & Info */}
        <div className="flex items-center px-4 py-2 gap-2 border-b border-gray-200 text-xs justify-between">
          <button className="flex items-center gap-1 px-3 py-1 rounded-md bg-gray-100 hover:bg-gray-200 border border-gray-300">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-4 w-4"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              strokeWidth="2"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
            Clear All
          </button>
          <p className="text-gray-600">
            Pinned: <span className="font-medium">1</span>
          </p>
        </div>
      </section>

      {/* List */}
      <Clips items={items} />
    </main>
  );
}

export default App;
