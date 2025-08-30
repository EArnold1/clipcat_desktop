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

type PinAction = 'pin' | 'unpin';

type PinFn = { handlePin: (id: string, action: PinAction) => Promise<void> };

export {
  type Clip,
  type ClipsData,
  type TextClip,
  type ImageClip,
  type PinAction,
  type PinFn,
};
