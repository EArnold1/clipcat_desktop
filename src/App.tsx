import { useCallback, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import 'src/App.css';
import { Clips } from 'components/clip';
import { ClipsData, type Clip } from 'src/types/clip';
import { useTauriEventListener } from 'src/hooks/useTauriListener';

function App() {
  const [items, setItems] = useState<ClipsData>({
    pinned_clips: [],
    mem_clips: [],
  });

  const handleEvent = useCallback((payload: Clip) => {
    setItems((prev) => ({ ...prev, mem_clips: [payload, ...prev.mem_clips] }));
  }, []);

  useTauriEventListener<Clip>({
    eventName: 'new_clip',
    handlePayload: handleEvent,
  });

  // const { payload } = useTauriEventListener<string>({
  //   eventName: 'error',
  // });

  const loadClips = async () => {
    const clips = await invoke<ClipsData>('load_clips');

    setItems(clips);
  };

  const clearClips = async () => {
    setItems((prev) => ({ ...prev, mem_clips: [] }));
    await invoke('clear_clips');
    loadClips();
  };

  const handleClearClips = useCallback(() => {
    // skipcq: JS-0098
    void clearClips();
  }, [clearClips]);

  useEffect(() => {
    // skipcq: JS-0098
    void loadClips();
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
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth={1.5}
                stroke="currentColor"
                className="size-4"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                />
              </svg>
            </button>
          </form>
        </div>

        {/* Controls & Info */}
        <div className="flex items-center px-4 py-2 gap-2 border-b border-gray-200 text-xs justify-between">
          <button
            className="cursor-pointer flex items-center gap-1 px-3 py-1 rounded-md bg-gray-100 hover:bg-gray-200 border border-gray-300"
            onClick={handleClearClips}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={1.5}
              stroke="currentColor"
              className="size-4"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
              />
            </svg>
            Clear All
          </button>
          <p className="text-gray-600">
            Pinned: <span className="font-medium">0</span>
          </p>
        </div>
      </section>

      {/* List */}

      <Clips {...items} />
    </main>
  );
}

export default App;
