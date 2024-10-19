import { OutputFormat, PollyClient, SynthesizeSpeechCommand, VoiceId } from "@aws-sdk/client-polly";
import { TranslateClient, TranslateTextCommand } from "@aws-sdk/client-translate";
import { readTextFile, remove } from "@tauri-apps/plugin-fs";

const maleVoicesByLanguage: { [key: string]: string } = {
  "ar": "Zeina",    // Arabic (although Zeina is female, it's the only Arabic voice)
  "cmn": "Zhiyu",   // Chinese Mandarin
  "da": "Mads",     // Danish
  "nl": "Ruben",    // Dutch
  "en": "Matthew",  // English (you might want to specify en-US, en-GB, etc.)
  "fr": "Mathieu",  // French
  "de": "Hans",     // German
  "hi": "Aditi",    // Hindi (although Aditi is female, it's the only Hindi voice)
  "is": "Karl",     // Icelandic
  "it": "Giorgio",  // Italian
  "ja": "Takumi",   // Japanese
  "ko": "Seoyeon",  // Korean (although Seoyeon is female, it's the only Korean voice)
  "nb": "Liv",      // Norwegian (although Liv is female, it's the only Norwegian voice)
  "pl": "Jacek",    // Polish
  "pt": "Cristiano",// Portuguese
  "ro": "Carmen",   // Romanian (although Carmen is female, it's the only Romanian voice)
  "ru": "Maxim",    // Russian
  "es": "Enrique",  // Spanish
  "sv": "Astrid",   // Swedish (although Astrid is female, it's the only Swedish voice)
  "tr": "Filiz",    // Turkish (although Filiz is female, it's the only Turkish voice)
  "cy": "Gwyneth",  // Welsh (although Gwyneth is female, it's the only Welsh voice)
};

async function convertPollyStreamToAudioBlob(stream: ReadableStream): Promise<Blob> {
  // Create a new ReadableStream from the input stream
  const reader = stream.getReader();
  const chunks: Uint8Array[] = [];

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
  }

  // Concatenate all chunks into a single Uint8Array
  const totalLength = chunks.reduce((acc, chunk) => acc + chunk.length, 0);
  const audioData = new Uint8Array(totalLength);
  let offset = 0;
  for (const chunk of chunks) {
    audioData.set(chunk, offset);
    offset += chunk.length;
  }

  // Create and return a Blob with the audio data
  return new Blob([audioData], { type: 'audio/mpeg' });
}

export const getSpeechFromText = async (client: PollyClient, text: string, language: string) => {
  const params = {
    Text: text,
    OutputFormat: OutputFormat.PCM,
    VoiceId: maleVoicesByLanguage[language] as VoiceId
  };

  try {
    const command = new SynthesizeSpeechCommand(params);
    const data = await client.send(command);

    const audioBlob = await convertPollyStreamToAudioBlob(data.AudioStream!);
    return audioBlob;
  } catch (error) {
    console.error("Error:", error);
  }
}

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

