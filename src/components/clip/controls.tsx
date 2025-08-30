import { useCallback } from 'react';
import { ClipFns } from 'src/types/clip';
import { IconButton } from 'src/components/icon-button';

type Props = {
  id: string;
  isPinned?: boolean;
} & ClipFns;

export const Controls = ({ id, isPinned, handlePin, handleDelete }: Props) => {
  const handlePinClick = useCallback(
    () => handlePin(id, isPinned ? 'unpin' : 'pin'),
    [handlePin, id, isPinned]
  );
  const handleDeleteClick = useCallback(
    () => handleDelete?.(id),
    [handleDelete, id]
  );

  return (
    <div className="flex gap-x-1 items-center">
      <IconButton
        aria-label="Pin"
        size="sm"
        variant="plain"
        className="hover:bg-gray-100 group"
        tooltip={isPinned ? 'Unpin clip' : 'Pin clip'}
        onClick={handlePinClick}
      >
        {isPinned ? (
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            className="size-4 text-yellow-400"
          >
            <path
              fillRule="evenodd"
              d="M6.32 2.577a49.255 49.255 0 0 1 11.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 0 1-1.085.67L12 18.089l-7.165 3.583A.75.75 0 0 1 3.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93Z"
              clipRule="evenodd"
            />
          </svg>
        ) : (
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            strokeWidth={1.5}
            stroke="currentColor"
            className="size-4 group-hover:text-yellow-400"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0Z"
            />
          </svg>
        )}
      </IconButton>

      {!isPinned && (
        <IconButton
          aria-label="Delete"
          size="sm"
          variant="plain"
          tooltip="Pin clip"
          className="hover:bg-gray-100"
          onClick={handleDeleteClick}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            strokeWidth={1.5}
            stroke="currentColor"
            className="size-4 text-red-500"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
            />
          </svg>
        </IconButton>
      )}
    </div>
  );
};
