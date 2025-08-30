import { ClipFns, type TextClip } from 'src/types/clip';
import { ClipCard } from 'src/components/clip/card';

type TextClipProps = TextClip & {
  handleCopy: (id: string) => Promise<void>;
  isPinned?: boolean;
} & ClipFns;

export const TextClipItem = ({
  Text: { id, value },
  ...others
}: TextClipProps) => {
  return (
    <ClipCard
      icon={
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="size-4 text-indigo-600"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z"
          />
        </svg>
      }
      id={id}
      {...others}
    >
      <p className="text-[13px] text-gray-900 line-clamp-2">{value}</p>
      <p className="text-[11px] text-gray-500 mt-1">Text ·</p>
    </ClipCard>
  );
};
