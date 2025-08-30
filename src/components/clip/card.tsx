import { ClipFns } from 'src/types/clip';
import { Controls } from 'src/components/clip/controls';

type Props = {
  children: React.ReactNode;
  icon: React.ReactNode;
  isPinned?: boolean;
  id: string;
  handleCopy: (id: string) => Promise<void>;
} & ClipFns;

export const ClipCard = ({
  children,
  id,
  icon,
  handleCopy,
  ...others
}: Props) => {
  return (
    // skipcq: JS-0746, JS-0765
    <div
      className="flex items-start gap-3 px-4 py-3 border-b border-gray-200 hover:bg-gray-50 cursor-pointer"
      onClick={() => handleCopy(id)}
    >
      <div className="flex-shrink-0 mt-1">{icon}</div>
      <div className="flex-1 min-w-0 flex flex-col gap-2">{children}</div>
      <Controls id={id} {...others} />
    </div>
  );
};
