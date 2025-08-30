import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';
import { Clip, ClipFns } from 'src/types/clip';
import { TextClipItem } from 'src/components/clip/text-clip';
import { ImageClipItem } from 'src/components/clip/image-clip';

type Props = {
  clip: Clip;
  isPinned?: boolean;
} & ClipFns;

export const ClipItem = ({
  clip,
  isPinned,
  handlePin,
  handleDelete,
}: Props) => {
  const handleCopy = useCallback(async (id: string) => {
    await invoke('copy_clip', { id });
  }, []);

  if ('Image' in clip)
    return (
      <ImageClipItem
        {...clip}
        isPinned={isPinned}
        handleCopy={handleCopy}
        handlePin={handlePin}
        handleDelete={handleDelete}
      />
    );

  return (
    <TextClipItem
      {...clip}
      isPinned={isPinned}
      handleCopy={handleCopy}
      handlePin={handlePin}
      handleDelete={handleDelete}
    />
  );
};
