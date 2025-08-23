type ClipItem = {
  id: string;
  value: string;
};

type TextClip = { Text: ClipItem };
type ImageClip = { Image: { path: string } };

type Clip = TextClip | ImageClip;

type ClipsData = {
  pinned_clips: Clip[];
  mem_clips: Clip[];
};

export { type Clip, type ClipsData, type TextClip, type ImageClip };
