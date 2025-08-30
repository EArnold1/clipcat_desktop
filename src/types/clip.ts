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

type ClipFns = {
  handlePin: (id: string, action: PinAction) => Promise<void>;

  handleDelete?: (id: string) => Promise<void>;
};

export {
  type Clip,
  type ClipsData,
  type TextClip,
  type ImageClip,
  type PinAction,
  type ClipFns,
};
