import { useEffect, useState } from "react";
import "./App.css";
import { readDir, readFile, readTextFile } from "@tauri-apps/plugin-fs";

interface SegmentJson {
  text: string;
  media_path: string;
}

const SEGMENTS_DIR = '/home/anon/.flashcard/segments';

function App() {
  const [subtitle, setSubtitle] = useState<string>('');
  const [segments, setSegments] = useState<string[]>([]);
  const [index, setIndex] = useState<number>(0);
  const [videoUrl, setVideoUrl] = useState<string>('');

  const loadVideo = async (path: string) => {
      try {
        const jsonFile = await readTextFile(path);
        const parsedJson: SegmentJson = JSON.parse(jsonFile);
        setSubtitle(parsedJson.text);

        const videoFile = await readFile(parsedJson.media_path);
        // TODO get type from extension
        const blob = new Blob([videoFile], { type: 'video/mp4' });
        const videoUrl = URL.createObjectURL(blob);

        setVideoUrl(videoUrl);

      } catch (error) {
        console.error('Failed to load video:', error);
      }
  }

  useEffect(() => {
    readDir(SEGMENTS_DIR)
    .then(entries => {
      console.log(entries);
      setSegments(entries.map(entry => entry.name));
      loadVideo(`${SEGMENTS_DIR}/${entries[0].name}`);
    });
  }, []);

  const handleNext = async (e: any) => {
    e.preventDefault();
    console.log('handleNext', index);
    await loadVideo(`${SEGMENTS_DIR}/${segments[index + 1]}`);
    setIndex(prevIndex => prevIndex+1);
  }

  return (
 
     <div className="h-screen w-screen flex flex-col">
    <video controls src={videoUrl} />
      <div className="text-2xl">{subtitle}</div>
      <button onClick={handleNext}>Next</button>
    </div> 
  );
}

export default App;
