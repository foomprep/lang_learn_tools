import { useEffect, useState } from "react";
import "./App.css";
import { readDir, readFile, readTextFile } from "@tauri-apps/plugin-fs";
import { deleteSegment, getTranslation } from "./utils";

interface SegmentJson {
  text: string;
  media_path: string;
}

interface Word {
  translation: string;
  text: string;
}

const SEGMENTS_DIR = '/home/anon/.flashcard/segments';

function App() {
  const [subtitle, setSubtitle] = useState<string>('');
  const [segments, setSegments] = useState<string[]>([]); // List of file names in SEGMENTS_DIR
  const [index, setIndex] = useState<number>(0);
  const [videoUrl, setVideoUrl] = useState<string>('');
  const [translation, setTranslation] = useState<string>('');
  const [word, setWord] = useState<Word>({translation: '', text: ''});

  const loadVideo = async (path: string) => {
      try {
        const jsonFile = await readTextFile(path);
        const parsedJson: SegmentJson = JSON.parse(jsonFile);
        setSubtitle(parsedJson.text);
        setTranslation(await getTranslation(parsedJson.text));

        const videoFile = await readFile(parsedJson.media_path);
        // TODO get type from extension
        const blob = new Blob([videoFile], { type: 'video/mp4' });
        const videoUrl = URL.createObjectURL(blob);

        setVideoUrl(videoUrl);

      } catch (error) {
        console.error('Failed to load video:', error);
      }
  }

  const handleClick = async (word: string) => {
    const translation = await getTranslation(word);
    setWord({
      text: word,
      translation: translation,
    });
  }

  useEffect(() => {
    readDir(SEGMENTS_DIR)
      .then(entries => {
        const shuffledEntries = entries.sort(() => Math.random() - 0.5);
        setSegments(shuffledEntries.map(entry => entry.name));
        loadVideo(`${SEGMENTS_DIR}/${shuffledEntries[0].name}`);
      });
  }, []);

  const handleNext = async (e: any) => {
    e.preventDefault();
    await loadVideo(`${SEGMENTS_DIR}/${segments[index + 1]}`);
    setIndex(prevIndex => prevIndex+1);
  }

  const handleDelete = async (e: any) => {
    const currentSegment = segments[index];
    await deleteSegment(`${SEGMENTS_DIR}/${currentSegment}`);
    await handleNext(e);
  }

  return (
    <div className="h-screen w-screen p-4 flex flex-col gap-3 text-2xl items-center justify-center">
      <video className="h-1/2" controls preload="auto" src={videoUrl} />
      <div className="flex gap-3">
        <button onClick={handleNext}>Next</button>
        <button onClick={handleDelete}>Remove</button>
      </div>
      <div className="flex gap-2 w-full">
        <div className="flex-grow w-1/2">
          <div className="flex flex-wrap gap-2 items-center text-4xl">
            {subtitle.split(' ').map(word => {
              return <div onClick={() => handleClick(word)} className="cursor-pointer">{word}</div>
            })}
          </div>
          <div>{translation}</div>
        </div>
        <div className="flex-grow w-1/2">
          <div>{word.text}</div>
          <div>{word.translation}</div>
        </div>
      </div>
    </div> 
  );

}

export default App;
