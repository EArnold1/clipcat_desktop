import { ClipsData, PinFn } from 'src/types/clip';
import { ClipItem } from './clip-item';

export const Clips = ({
  pinned_clips,
  mem_clips,
  handlePin,
}: ClipsData & PinFn) => {
  const isEmpty = ![...pinned_clips, ...mem_clips].length;

  return (
    <section className="mt-[98px] h-full">
      {pinned_clips.length
        ? pinned_clips.map((clip) => (
            <ClipItem
              clip={clip}
              key={'Image' in clip ? clip.Image.path : clip.Text.id}
              handlePin={handlePin}
              isPinned
            />
          ))
        : null}

      {mem_clips.length
        ? mem_clips.map((clip) => (
            <ClipItem
              clip={clip}
              key={'Image' in clip ? clip.Image.path : clip.Text.id}
              handlePin={handlePin}
            />
          ))
        : null}

      {isEmpty && (
        <div className="p-6 text-center text-sm text-gray-500">
          No clipboard history yet. Copy something to get started.
        </div>
      )}
    </section>
  );
};
