type ClipItem = {
  id: string;
  value: string;
};

type ClipsData = {
  pinned_clips: ClipItem[];
  mem_clips: ClipItem[];
};

export { type ClipItem, type ClipsData };
