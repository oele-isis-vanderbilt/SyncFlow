'use server';
import { z } from 'zod';
import {
  NewApiKeyRequest,
  NewApiKeySchema,
  NewProjectSchema,
  NewSession,
  NewSessionSchema,
  projectClient,
} from './project-client';
import { redirect } from 'next/navigation';
import { revalidatePath } from 'next/cache';
import { Project } from '@/types/project';

export type FormSubmissionState<T> = {
  errors?: string[];
  success: boolean;
  data?: T;
};

export async function deleteProject(
  id: string,
): Promise<{ status: 'success' | 'error'; project?: Project; error?: string }> {
  let result = (await projectClient.deleteProject(id))
    .map((project) => {
      revalidatePath('/dashboard');
      revalidatePath('/dashboard/projects');
      return {
        status: 'success',
        project: project,
      };
    })
    .ok()
    .unwrapOrElse((error) => {
      return {
        status: 'error',
        error: JSON.stringify(error, null, 2),
      };
    });

  return result;
}

export async function createProject(
  prevState: FormSubmissionState<Project> | null,
  formData: FormData,
) {
  let createProjectRequest = Object.fromEntries(formData.entries());
  let result = NewProjectSchema.safeParse(createProjectRequest);
  if (!result.success) {
    return {
      success: false,
      errors: result.error.issues.map((issue) => issue.message),
    };
  } else {
    let project = result.data;
    project.storageType = project.storageType || 's3';
    const createResult = await projectClient.createProject(project);
    const state = createResult
      .map((project) => {
        revalidatePath('/dashboard');
        revalidatePath('/dashboard/projects');
        return {
          success: true,
          data: project,
        };
      })
      .unwrapOrElse((error) => {
        return {
          success: false,
          data: null as any, // or you can use {} if Project is an object type
          errors: [
            `An error occurred while creating the project. ${error.message} | Code: ${error.code}`,
          ],
        };
      });

    if (state.success) {
      redirect(`/dashboard/projects/${state.data.id}`);
    } else {
      return state;
    }
  }
}

export async function createProjectSession(
  projectId: string,
  prevState: FormSubmissionState<NewSession> | null,
  formData: FormData,
) {
  if (formData === null) {
    return null;
  }
  const createSessionRequest = Object.fromEntries(formData.entries());
  const result = NewSessionSchema.safeParse(createSessionRequest);
  if (!result.success) {
    revalidatePath(`/dashboard/projects/${projectId}`);
    return {
      success: false,
      errors: result.error.issues.map((issue) => issue.message),
    };
  } else {
    const sessionRequest = result.data;
    const state = (await projectClient.createSession(projectId, sessionRequest))
      .map((session) => {
        revalidatePath(`/dashboard/projects/${projectId}`);
        return {
          success: true,
          data: session,
        };
      })
      .unwrapOrElse((error) => {
        return {
          success: false,
          data: null as any,
          errors: [
            `An error occurred while creating the session. ${error.message} | Code: ${error.code}`,
          ],
        };
      });

    return state;
  }
}

export async function stopSession(projectId: string, sessionId: string) {
  let result = (await projectClient.stopSession(projectId, sessionId))
    .map((session) => {
      revalidatePath(`/dashboard/projects/${projectId}`);
      return {
        success: true,
        data: session,
      };
    })
    .unwrapOrElse((error) => {
      return {
        success: false,
        error: JSON.stringify(error, null, 2),
      };
    });

  return result;
}

export async function getSessionParticipants(
  projectId: string,
  sessionId: string,
) {
  return projectClient.listParticipants(projectId, sessionId);
}

export async function deleteSession(projectId: string, sessionId: string) {
  let result = (await projectClient.deleteSession(projectId, sessionId))
    .map((session) => {
      revalidatePath(`/dashboard/projects/${projectId}`);
      return {
        success: true,
        data: session,
      };
    })
    .unwrapOrElse((error) => {
      return {
        success: false,
        error: JSON.stringify(error, null, 2),
      };
    });
  return result;
}

export async function createApiKey(
  projectId: string,
  prevState: FormSubmissionState<NewApiKeyRequest> | null,
  formData: FormData,
) {
  if (formData === null) {
    return null;
  }
  const createApiKeyRequest = Object.fromEntries(formData.entries());
  const result = NewApiKeySchema.safeParse(createApiKeyRequest);

  if (!result.success) {
    revalidatePath(`/dashboard/projects/${projectId}`);
    return {
      success: false,
      errors: result.error.issues.map((issue) => issue.message),
    };
  } else {
    const apiKeyRequest = result.data;
    const state = (await projectClient.createApiKeys(projectId, apiKeyRequest))
      .map((apiKey) => {
        revalidatePath(`/dashboard/projects/${projectId}`);
        return {
          success: true,
          data: apiKey,
        };
      })
      .unwrapOrElse((error) => {
        return {
          success: false,
          data: null as any,
          errors: [
            `An error occurred while creating the API key. ${error.message} | Code: ${error.code}`,
          ],
        };
      });

    return state;
  }
}

export async function deleteApiKey(projectId: string, apiKeyId: string) {
  let result = (await projectClient.deleteApiKey(projectId, apiKeyId))
    .map((apiKey) => {
      revalidatePath(`/dashboard/projects/${projectId}`);
      return {
        success: true,
        data: apiKey,
      };
    })
    .unwrapOrElse((error) => {
      return {
        success: false,
        error: JSON.stringify(error, null, 2),
      };
    });

  return result;
}

export async function deleteDevice(projectId: string, deviceId: string) {
  let result = (await projectClient.deleteDevice(projectId, deviceId))
    .map((device) => {
      revalidatePath(`/dashboard/projects/${projectId}`);
      return {
        success: true,
        data: device,
      };
    })
    .unwrapOrElse((error) => {
      return {
        success: false,
        error: JSON.stringify(error, null, 2),
      };
    });

  return result;
}
