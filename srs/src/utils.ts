import { readTextFile, remove } from "@tauri-apps/plugin-fs";

export const getTranslation = async (text: string, language: string) => {
    try {
      const response = await fetch('https://api.openai.com/v1/chat/completions', {
        method: 'POST',
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${import.meta.env.VITE_OPENAI_API_KEY}`
        },
        body: JSON.stringify({
          "model": "gpt-3.5-turbo",
          "messages": [{"role": "user", "content": `Return a translation into English of ${language} text: ${text}`}]
        })
      });

      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }

      const responseJson = await response.json();
      return responseJson.choices[0].message.content;
    } catch (error: any) {
      console.error(error.message);
    }
}

export const deleteSegment = async (segmentPath: string): Promise<void> => {
  try {
    // Read the JSON file
    const jsonContent = await readTextFile(segmentPath);
    
    // Parse the JSON content
    const jsonData = JSON.parse(jsonContent);
    
    // Check if media_path property exists
    if (!jsonData.hasOwnProperty('media_path')) {
      throw new Error('media_path property not found in JSON');
    }
    
    // Delete the media file
    await remove(jsonData.media_path);
    console.log(`Deleted media file: ${jsonData.media_path}`);
    
    // Delete the JSON file
    await remove(segmentPath);
    console.log(`Deleted JSON file: ${segmentPath}`);
  } catch (error) {
    console.error('An error occurred:', error);
    throw error;
  }
}

export const removePunc = (word: string) => {
    return word.replace(/[!?¿".,]/g, '');
}

