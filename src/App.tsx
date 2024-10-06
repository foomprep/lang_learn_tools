import { useEffect } from "react";
import "./App.css";
import { readFile, readTextFile } from "@tauri-apps/plugin-fs";

interface SegmentJson {
  text: string;
  media_path: string;
}

function App() {
  useEffect(() => {
    const loadVideo = async () => {
        try {
          const jsonFile = await readTextFile('/home/anon/.flashcard/Scheherazade.1963.1080p.BluRay.x264.AAC5.1-[YTS.MX]_2693.75.json');
          const parsedJson: SegmentJson = JSON.parse(jsonFile);
          console.log(parsedJson.text);
          console.log(parsedJson.media_path);

          const videoFile = await readFile(parsedJson.media_path);
          const blob = new Blob([videoFile], { type: 'video/mp4' });
          const videoUrl = URL.createObjectURL(blob);

          const videoElement = document.getElementById('video-player') as HTMLVideoElement;
          videoElement.src = videoUrl;

        } catch (error) {
          console.error('Failed to load video:', error);
        }
    }
    loadVideo();
  }, []);

  return (
    <div>
      <video controls id="video-player" />
    </div> 
  );
}

export default App;
