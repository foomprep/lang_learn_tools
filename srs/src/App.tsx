import { useEffect, useState } from "react";
import "./App.css";
import { readDir, readFile, readTextFile } from "@tauri-apps/plugin-fs";
import { deleteSegment, getTranslation, removePunc } from "./utils";

interface SegmentJson {
  text: string;
  media_path: string;
  language: string;
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
  const [translation, setTranslation] = useState<string>('');
  const [word, setWord] = useState<Word>({translation: '', text: ''});
  const [language, setLanguage] = useState<string>('');
  const [videoUrl, setVideoUrl] = useState<string>('');
  const [playbackRate, setPlaybackRate] = useState<string>("Normal");

  const keyPress = async (event: any) => {
    if (event.key === 'n' || event.key === 'N') {
        await handleNext(event);
    }
  }

  useEffect(() => {
    document.addEventListener('keydown', keyPress);
    return () => document.removeEventListener("keydown", keyPress);
  });

  const getSelectedText = () => {
    if (typeof window.getSelection != "undefined") {
      if (window.getSelection()) {
        return window.getSelection()?.toString();
      }
    } 
    return null;
  }

  function doSomethingWithSelectedText() {
    var selectedText = getSelectedText();
    if (selectedText) {
      handleTranslation(selectedText);
    }
  }

  document.onmouseup = doSomethingWithSelectedText;

  const loadVideo = async (path: string) => {
      try {
        const jsonFile = await readTextFile(path);
        const parsedJson: SegmentJson = JSON.parse(jsonFile);
        setSubtitle(parsedJson.text);
        setLanguage(parsedJson.language);
        setTranslation(await getTranslation(parsedJson.text, parsedJson.language));

        const videoFile = await readFile(parsedJson.media_path);
        // TODO get type from extension
        const blob = new Blob([videoFile], { type: 'video/mp4' });
        const videoUrl = URL.createObjectURL(blob);

        setVideoUrl(videoUrl);

      } catch (error) {
        console.error('Failed to load video:', error);
      }
  }

  const handleTranslation = async (word: string) => {
    const translation = await getTranslation(removePunc(word), language);
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

  const handlePlayback = (e: any) => {
    const video = document.getElementById('player');
    console.log("Video loaded");
    console.log("playbackRate property:", video!.playbackRate);
    if (video!.playbackRate === 1.0) {
      video!.playbackRate = 0.5;
      setPlaybackRate("Slow");
    } else {
      video!.playbackRate = 1.0;
      setPlaybackRate("Normal");
    }
  }

  return (
    <div className="h-screen w-screen p-6 flex gap-3 text-2xl box-border">
      <div className="flex flex-col gap-2 items-center justify-center w-1/2">
        <video width="100%" id="player" controls preload="auto" src={videoUrl} />
        <div className="flex gap-3">
          <button type="button" onClick={handlePlayback} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">{playbackRate}</button>
          <button type="button" onClick={handleNext} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">Next</button>
          <button type="button" onClick={handleDelete} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">Remove</button>
        </div>
      </div>
      <div className="flex flex-col gap-2 w-1/2 h-full">
        <div className="h-1/2">
          <div>{language}</div>
          <div className="flex flex-wrap gap-2 items-center text-4xl">
            {subtitle.split(' ').map(word => {
              return <div onClick={() => handleTranslation(word)} className="cursor-pointer">{word}</div>
            })}
          </div>
          <div>{translation}</div>
        </div>
        <div className="h-1/2">
          <div>{word.text}</div>
          <div>{word.translation}</div>
        </div>
      </div>
    </div> 
  );

}

export default App;
