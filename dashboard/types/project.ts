/*
    This file contains the types for the project.
    {
"bucketName": "string",
"description": "string",
"endpoint": "string",
"id": "string",
"livekitServerUrl": "string",
"name": "string",
"storageType": "string"
}
*/

export interface Project {
  id: string;
  name: string;
  description: string;
  livekitServerUrl: string;
  endpoint: string;
  bucketName: string;
  storageType: string;
  lastUpdated: number;
}
