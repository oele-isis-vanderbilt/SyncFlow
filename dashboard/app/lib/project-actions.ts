'use server';
import { z } from 'zod';
import { NewProjectSchema, projectClient } from './project-client';
import { redirect } from 'next/navigation';
import { revalidatePath } from 'next/cache';
import { Project } from '@/types/project';
import { ClientError } from './auth-http-client';

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
