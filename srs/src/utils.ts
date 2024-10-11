import { TranslateClient, TranslateTextCommand } from "@aws-sdk/client-translate";
import { readTextFile, remove } from "@tauri-apps/plugin-fs";

export const getTranslation = async (
  client: TranslateClient, 
  text: string, 
  sourceLanguage: string, 
  targetLanguage: string
) => {
    try {
      const input = {
        Text: text,
        SourceLanguageCode: sourceLanguage,
        TargetLanguageCode: targetLanguage,
      };
    
      const command = new TranslateTextCommand(input);
    
      try {
        const response = await client.send(command);
        console.log(response);
        return response.TranslatedText;
      } catch (error) {
        console.error("Error translating text:", error);
        throw error;
      }
    } catch (error) {
      console.error(error);
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

