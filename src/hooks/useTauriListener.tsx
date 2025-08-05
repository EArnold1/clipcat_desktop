import { useEffect, useState, useRef, useCallback } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

type Params<T> = {
  eventName: string;
  delay?: number; // ms
  handlePayload?: (payload: T) => void;
};

export function useTauriEventListener<T>({
  eventName,
  handlePayload,
  delay = 500,
}: Params<T>) {
  const [payload, setPayload] = useState<T | null>(null);
  const unlistenRef = useRef<UnlistenFn | null>(null);
  const debounceTimer = useRef<number | null>(null);
  const latestIncoming = useRef<T | null>(null);
  const mountedRef = useRef(true);

  const flush = useCallback(() => {
    if (latestIncoming.current !== null) {
      setPayload(latestIncoming.current);
      handlePayload?.(latestIncoming.current);
      latestIncoming.current = null;
    }
    if (debounceTimer.current !== null) {
      clearTimeout(debounceTimer.current);
      debounceTimer.current = null;
    }
  }, []);

  useEffect(() => {
    mountedRef.current = true;

    listen<T>(eventName, (event) => {
      if (!mountedRef.current) return;
      latestIncoming.current = event.payload;

      if (debounceTimer.current !== null) {
        clearTimeout(debounceTimer.current);
      }

      // schedule flush after delay
      debounceTimer.current = window.setTimeout(() => {
        flush();
      }, delay);
    })
      .then((unlisten) => {
        unlistenRef.current = unlisten;
      })
      .catch((err) => {
        console.error(`Failed to listen to ${eventName}:`, err);
      });

    return () => {
      mountedRef.current = false;
      if (debounceTimer.current !== null) {
        clearTimeout(debounceTimer.current);
      }
      if (unlistenRef.current) {
        unlistenRef.current();
      }
    };
  }, [eventName, delay, flush]);

  return { payload };
}
