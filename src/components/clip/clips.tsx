import { ClipsData, ClipFns, type Clip } from 'src/types/clip';
import { ClipItem } from 'src/components/clip/clip-item';
import { MAX_SEARCH_LENGTH } from 'src/App';

type Props = {
  isPinned?: boolean;
  clips: Array<Clip>;
} & ClipFns;

const List = ({ clips, ...others }: Props) => {
  return clips.length
    ? clips.map((clip) => (
        <ClipItem
          clip={clip}
          key={'Image' in clip ? clip.Image.path : clip.Text.id}
          {...others}
        />
      ))
    : null;
};

type ClipsProps = {
  searchResult: Array<Clip>;
  searchQuery: string;
} & ClipsData &
  ClipFns;

export const Clips = ({
  pinned_clips,
  mem_clips,
  searchQuery,
  searchResult,
  handlePin,
  handleDelete,
}: ClipsProps) => {
  const isEmpty = ![...pinned_clips, ...mem_clips].length;

  return (
    <section className="mt-[102px] h-full">
      <List isPinned clips={pinned_clips} handlePin={handlePin} />

      {searchQuery.length >= MAX_SEARCH_LENGTH ? (
        searchResult.length ? (
          <List
            clips={searchResult}
            handlePin={handlePin}
            handleDelete={handleDelete}
          />
        ) : (
          <div className="p-6 text-center text-sm text-gray-500">
            No search result.
          </div>
        )
      ) : (
        <List
          clips={mem_clips}
          handlePin={handlePin}
          handleDelete={handleDelete}
        />
      )}

      {isEmpty && (
        <div className="p-6 text-center text-sm text-gray-500">
          No clipboard history yet. Copy something to get started.
        </div>
      )}
    </section>
  );
};
