import { useCallback, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import 'src/App.css';
import { Clips } from 'src/components/clip/clips';
import { ClipsData, PinAction, type Clip } from 'src/types/clip';
import { useTauriEventListener } from 'src/hooks/useTauriListener';
import { Header } from 'src/components/header';

/// max number of items in the memory clip store
const MAX_LENGTH = 10;

export const MAX_SEARCH_LENGTH = 3;

const App = () => {
  const [items, setItems] = useState<ClipsData>({
    pinned_clips: [],
    mem_clips: [],
  });

  const [searchResult, setSearchResult] = useState<Clip[]>([]);

  const [query, setQuery] = useState('');

  const handleEvent = useCallback((payload: Clip) => {
    setItems((prev) => {
      const mem_clips = prev.mem_clips;
      // using 10 because that is the limit
      // if a settings page is added, the value will be dynamically set
      if (mem_clips.length >= MAX_LENGTH) {
        mem_clips.pop();
      }
      return {
        pinned_clips: prev.pinned_clips,
        mem_clips: [payload, ...mem_clips],
      };
    });
  }, []);

  useTauriEventListener<Clip>({
    eventName: 'new_clip',
    handlePayload: handleEvent,
  });

  const loadClips = async () => {
    const clips = await invoke<ClipsData>('load_clips');

    setItems(clips);
  };

  const clearClips = async () => {
    setItems((prev) => ({ ...prev, mem_clips: [] }));
    await invoke('clear_clips');
    loadClips();
  };

  const handlePin = useCallback(async (id: string, action: PinAction) => {
    if (action === 'pin') {
      await invoke('pin_clip', { id });
    } else {
      await invoke('unpin_clip', { id });
    }
    loadClips();
  }, []);

  const handleDelete = useCallback(async (id: string) => {
    setItems((prev) => {
      const mem_clips = prev.mem_clips.filter((clip) => {
        if ('Image' in clip) {
          return clip.Image.path !== id;
        } else {
          return clip.Text.id !== id;
        }
      });

      return { pinned_clips: prev.pinned_clips, mem_clips };
    });

    await invoke('delete_clip', { id });

    loadClips();
  }, []);

  const handleSearch = useCallback(
    (value: string) => {
      setQuery(value);
    },
    [query]
  );

  const searchClip = async (query: string) => {
    const clips = await invoke<Clip[]>('search_clips', { query });

    setSearchResult(clips);
  };

  const handleClearClips = useCallback(() => {
    // skipcq: JS-0098
    void clearClips();
  }, [clearClips]);

  useEffect(() => {
    // skipcq: JS-0098
    void loadClips();
  }, []);

  useEffect(() => {
    if (query.trim().length >= MAX_SEARCH_LENGTH) {
      searchClip(query);
    }
  }, [query]);

  return (
    <main className="w-full overflow-hidden relative">
      <Header
        pinnedCount={items.pinned_clips.length}
        handleClearClips={handleClearClips}
        handleSearch={handleSearch}
      />

      <Clips
        {...items}
        handlePin={handlePin}
        handleDelete={handleDelete}
        searchQuery={query}
        searchResult={searchResult}
      />
    </main>
  );
};

export default App;
