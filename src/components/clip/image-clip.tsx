import { convertFileSrc } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { ImageClip, ClipFns } from 'src/types/clip';
import { appDataDir, join } from '@tauri-apps/api/path';
import { ClipCard } from 'src/components/clip/card';

type ImageClipProps = ImageClip & {
  isPinned?: boolean;
  handleCopy: (id: string) => Promise<void>;
} & ClipFns;

export const ImageClipItem = ({
  Image: { path },
  ...others
}: ImageClipProps) => {
  const [assetUrl, setAssetUrl] = useState<string | null>(null);
  const loadSrc = async () => {
    const appDataDirPath = await appDataDir();

    const filePath = await join(appDataDirPath, `images/${path}`);

    const assetUrl = convertFileSrc(filePath);

    setAssetUrl(assetUrl);
  };

  useEffect(() => {
    loadSrc();
  }, []);

  if (!assetUrl) return null;

  return (
    <ClipCard
      icon={
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="size-5 text-pink-600"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z"
          />
        </svg>
      }
      id={path}
      {...others}
    >
      <div className="grid grid-cols-1 gap-2">
        <div className="max-w-xs w-full">
          <div className="overflow-hidden rounded-md bg-white size-40">
            <img
              src={assetUrl}
              alt="Screenshot preview"
              className="object-cover"
            />
          </div>
        </div>
        <p className="text-[11px] text-gray-500 mt-1">Image ·</p>
      </div>
    </ClipCard>
  );
};
