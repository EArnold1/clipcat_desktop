import { IconButton } from 'src/components/icon-button';
import { Button } from 'src/components/button';

type Props = {
  pinnedCount: number;
  handleClearClips: () => void;
  handleSearch: (value: string) => void;
};

const SearchBar = ({ handleSearch }: Pick<Props, 'handleSearch'>) => {
  return (
    <div className="flex-1 flex gap-x-3 items-center border rounded-md bg-gray-50 border-gray-300 px-2">
      <input
        type="text"
        placeholder="Search"
        aria-label="Search clipboard"
        className="w-full text-xs outline-none py-2 bg-inherit"
        onChange={(e) => handleSearch(e.target.value)}
      />
      <IconButton
        className="flex items-center bg-none"
        variant="plain"
        size="sm"
        tooltip="Search"
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
            d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
          />
        </svg>
      </IconButton>
    </div>
  );
};

const Controls = ({
  pinnedCount,
  handleClearClips,
}: Omit<Props, 'handleSearch'>) => {
  return (
    <div className="flex items-center px-4 py-2 gap-2 border-b border-gray-200 text-xs justify-between">
      <Button
        variant="secondary"
        className="flex gap-x-2 items-center"
        size="sm"
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
      </Button>
      <p className="text-gray-600">
        Pinned: <span className="font-medium">{pinnedCount}</span>
      </p>
    </div>
  );
};

export const Header = ({
  pinnedCount,
  handleClearClips,
  handleSearch,
}: Props) => {
  return (
    <section className="fixed w-full bg-white z-20">
      {/* Header */}
      <div className="flex items-center px-4 py-3 border-b border-gray-200 gap-2">
        <h4 className="flex-1 text-sm font-semibold">Clipcat</h4>
        <SearchBar handleSearch={handleSearch} />
      </div>

      {/* Controls & Info */}
      <Controls pinnedCount={pinnedCount} handleClearClips={handleClearClips} />
    </section>
  );
};
