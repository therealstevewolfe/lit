import { Stitch, StitchToolClient } from "@google/stitch-sdk";
import { readFileSync } from "fs";

const ACCESS_TOKEN = process.env.STITCH_ACCESS_TOKEN;
const PROJECT_ID = "5859269315536512921";

const SCREENS = [
  { name: "01-onboarding-welcome", id: "df4b6f0328dd4d1dbff6fc776044ede0" },
  { name: "02-settings-models", id: "595d78ed072f49178d28f208dd63ac0b" },
  { name: "03-recording-overlay", id: "db5ffae8c6d94229b53c3dcbd76b92df" },
  { name: "04-settings-general", id: "2e749e6e1f5c4b9cb2c72ab4b9c5889a" },
  { name: "05-recording-overlay-mobile", id: "1429b6cc4b3e470a8fe1192f3ab9b4ba" },
  { name: "06-recording-overlay", id: "98a42da3321348f5881cefa4970a0b51" },
  { name: "07-user-profile", id: "c4962c7a8d784a5cbc2fa9aacda2c48c" },
  { name: "08-transcription-history", id: "3898c0b480194615a79ef828f4c5d80b" },
  { name: "09-audio-analysis", id: "8e7f532e7525449e905c2d050f9c659d" },
  { name: "10-privacy-policy", id: "4033c5ecb6714d97ad6cf3d8875ad381" },
  { name: "11-transcription-expanded-view", id: "87566c9338c746889d7f78ad5a5e4b37" },
  { name: "12-settings-general", id: "99c194a8e71045d69e28c0f99fc02d2b" },
  { name: "13-privacy-policy", id: "7b73dc7f98b44b8aa4112343204261f4" },
  { name: "14-terms-of-service", id: "4d125bea3b274565a4f670c796b9fd36" },
  { name: "15-settings-models", id: "c2ac6b96136241de91ac5722073eb8b0" },
  { name: "16-settings-trigger-setup", id: "590f95634e5346168c826ea895fd0ac1" },
];

async function main() {
  if (!ACCESS_TOKEN) {
    console.error("Error: STITCH_ACCESS_TOKEN environment variable is required");
    console.error("Get your OAuth access token from Google Cloud console or gcloud CLI:");
    console.error("  gcloud auth application-default print-access-token");
    process.exit(1);
  }

  const client = new StitchToolClient({
    accessToken: ACCESS_TOKEN,
    projectId: PROJECT_ID,
    baseUrl: "https://stitch.googleapis.com/mcp",
    timeout: 300_000,
  });

  const sdk = new Stitch(client);

  try {
    const project = sdk.project(PROJECT_ID);
    
    console.log(`Fetching ${SCREENS.length} screens from project ${PROJECT_ID}...`);
    
    for (const screen of SCREENS) {
      try {
        console.log(`Getting image for: ${screen.name} (${screen.id})`);
        
        // Create a new client for each screen to avoid connection reuse issues
        const screenClient = new StitchToolClient({
          accessToken: ACCESS_TOKEN,
          projectId: PROJECT_ID,
          baseUrl: "https://stitch.googleapis.com/mcp",
          timeout: 300_000,
        });
        const screenSdk = new Stitch(screenClient);
        const screenProject = screenSdk.project(PROJECT_ID);
        
        const screenObj = await screenProject.getScreen(screen.id);
        const imageUrl = await screenObj.getImage();
        console.log(`  Image URL: ${imageUrl}`);
        
        // Download the image
        const response = await fetch(imageUrl);
        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }
        
        const buffer = await response.arrayBuffer();
        const path = await import('path');
        const outputPath = path.join('stitch-screens', `${screen.name}.png`);
        const fs = await import('fs');
        fs.writeFileSync(outputPath, Buffer.from(buffer));
        console.log(`  Saved to: ${outputPath}`);
        
        await screenClient.close();
      } catch (err) {
        console.error(`  Error for ${screen.name}: ${err.message}`);
      }
    }
    
    console.log("Done!");
  } finally {
    await client.close();
  }
}

main().catch(console.error);