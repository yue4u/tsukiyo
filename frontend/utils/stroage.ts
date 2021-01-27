import firebase from "firebase/app";
import "firebase/storage";
import { v4 } from "uuid";

export const ogpStorage = firebase
  .app()
  .storage(("gs://" + import.meta.env.VITE_PUBLIC_BUCKET) as string)
  .ref()
  .child("ogp");

export const OGP = {
  async upload(file: File) {
    const ext = getExt(file.name);
    try {
      const filename = `event-${v4()}.${ext}`;
      const path = filename;
      await uploadTask(ogpStorage.child(filename).put(file));
      return path;
    } catch {
      // TODO: show message
      console.log("failed to upload");
    }
  },
  async delete(filename: string) {
    try {
      await ogpStorage.child(filename).delete();
    } catch {
      // TODO: show message
      console.log("failed to delete");
    }
  },
};

export const getExt = (fileName: string): string => {
  return fileName.includes(".") ? fileName.split(".").pop()!.toLowerCase() : "";
};

async function uploadTask(
  task: firebase.storage.UploadTask
): Promise<{
  id?: string;
  error?: Error;
}> {
  return new Promise((resolve, reject) => {
    task.on(
      "state_changed",
      (_snapshot) => {
        // Observe state change events such as progress, pause, and resume
        // Get task progress, including the number of bytes uploaded and the total number of bytes to be uploaded
        // const progress =
        //   (snapshot.bytesTransferred / snapshot.totalBytes) * 100;
        // switch (snapshot.state) {
        // case firebase.storage.TaskState.PAUSED: // or 'paused'
        // setTaskState("Upload is paused");
        // break;
        // case firebase.storage.TaskState.RUNNING: // or 'running'
        // setTaskState("Upload is running");
        // break;
        // }
      },
      (error) => {
        reject(error.message);
      },
      () => {
        task.snapshot.ref.getDownloadURL().then((_downloadURL) => {
          console.log("File available at", _downloadURL);
          resolve({});
        });
      }
    );
  });
}
